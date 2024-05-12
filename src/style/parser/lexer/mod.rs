mod location;
mod stream;
mod token;

use std::collections::VecDeque;

pub use location::SourceLocation;
use stream::Stream;
pub use token::*;

pub mod traits {
    pub trait Lexer<T>: Iterator<Item = T> {
        fn current(&self) -> Option<Self::Item>;
        fn rewind(&mut self);
    }
}

pub struct Lexer<'i> {
    pub(self) stream: Stream<'i>,
    pub(self) current: Option<Token>,
    pub(self) buffer: Vec<Token>,
}

impl<'i> Lexer<'i> {
    pub fn new(content: &'i str) -> Self {
        Self {
            stream: Stream::new(content),
            current: None,
            buffer: Vec::default(),
        }
    }
}

impl Lexer<'_> {
    /// Consume a comment
    fn consume_comment(&mut self) {
        while let Some(_) = self.stream.next() {
            if self.stream.peek::<0, 2>() == "*/" {
                self.stream.next();
                break;
            }
        }
    }

    /// Consume whitespaces
    #[inline]
    fn consume_whitespace(&mut self) {
        while let Some(c) = self.stream.next() {
            if !Self::is_whitespace_code_point(c) {
                self.stream.rewind();
                break;
            }
        }
    }

    /// Consume a <string-token>, or a <bad-string-token>.
    ///
    /// It expects the stream's next code point to be either " or '
    fn consume_string_token(&mut self) -> Token {
        let mut buf = String::default();
        let del = self.stream.next().unwrap();
        let location = self.stream.current_location;

        while let Some(c) = self.stream.next() {
            if c == del {
                return Token::string(buf, location);
            } else if c == '\\' && Self::is_valid_escape(self.stream.peek::<0, 2>()) {
                buf.push(self.stream.next().unwrap())
            } else if c == '\n' {
                return Token::bad_string(buf, location);
            } else {
                buf.push(c);
            }
        }

        Token::bad_string(buf, location)
    }

    /// Consume an ident sequence
    ///
    /// ```
    /// let mut lexer = Lexer::new("background-repeat 123456");
    /// let ident = lexer.consume_ident_sequence();
    /// assert_eq!(ident, "background-repeat");
    /// ```
    fn consume_ident_sequence(&mut self) -> String {
        let mut buf = String::default();

        while let Some(c) = self.stream.next() {
            if Self::is_ident_code_point(c) {
                buf.push(c)
            } else if Self::is_valid_escape(self.stream.peek::<0, 2>()) {
                buf.push(self.stream.next().unwrap())
            } else {
                self.stream.rewind();
                break;
            }
        }

        buf
    }

    /// Consume [0-9]+
    ///
    /// This method consumes a serie of digits until it reaches either EOF, or a non-digit code
    /// point.
    ///
    /// ```
    /// let mut lexer = Lexer::new("123456px");
    /// let digits = lexer.consume_digits();
    /// assert_eq!(digits, "123456".to_string());
    /// assert_eq!(lexer.stream.next(), Some('p'));
    /// ```
    pub(self) fn consume_digits(&mut self) -> String {
        let mut buf = String::default();

        while let Some(c) = self.stream.next() {
            if Self::is_digit_code_point(c) {
                buf.push(c)
            } else {
                self.stream.rewind();
                break;
            }
        }

        buf
    }

    /// Consume a number represented as (+|-)?[0-9]+("."[0-9]+)?(e(+|-)[0-9]+).
    ///
    /// # Example
    /// Example of number are -123.456e-789
    ///
    /// ```
    /// let mut lexer = Lexer::new("-123.456e-789");
    /// let number = lexer.consume_number();
    /// assert_eq!(number, Number::new("-123", "456", "789"));
    /// ```
    fn consume_number(&mut self) -> Number {
        let mut sel = 0;
        // Parts are : integer, decimal, exponent
        let mut parts: VecDeque<String> =
            VecDeque::from([String::default(), String::default(), String::default()]);

        while let Some(c) = self.stream.next() {
            if (c == '+' || c == '-') && (sel == 0 || sel == 2) && parts[sel].is_empty() {
                parts[sel].push(c);
            } else if Self::is_digit_code_point(c) {
                self.stream.rewind();
                parts[sel].push_str(&self.consume_digits());
            } else if Self::is_decimal_part_start(self.stream.peek::<0, 2>()) && sel == 0 {
                sel = 1;
            } else if Self::is_exponent_part_start(self.stream.peek::<0, 3>()) && sel != 2 {
                sel = 2;
            } else {
                self.stream.rewind();
                break;
            }
        }

        Number::new(
            parts.pop_front().unwrap(),
            parts.pop_front().unwrap(),
            parts.pop_front().unwrap(),
        )
    }

    /// Consume either a number, dimension or percentage token.
    fn consume_number_token(&mut self) -> Token {
        let location = self.stream.current_location.shift_right();
        let number = self.consume_number();

        if Self::is_ident_sequence_start(self.stream.peek::<1, 3>()) {
            let unit = self.consume_ident_sequence();
            Token::dimension(number, unit, location)
        } else if self.stream.peek::<1, 1>() == "%" {
            self.stream.next();
            Token::percentage(number, location)
        } else {
            Token::number(number, location)
        }
    }

    fn consume_remnants_of_bad_url(&mut self) {
        while let Some(c) = self.stream.next() {
            if c == ')' {
                return;
            }
        }
    }

    fn consume_url_token(&mut self, location: SourceLocation) -> Token {
        let mut buf = String::default();

        while let Some(c) = self.stream.next() {
            if Self::is_whitespace_code_point(c) {
                self.consume_whitespace();
            }

            if c == ')' {
                return Token::url(buf, location);
            } else if c == '"' || c == '\'' || !c.is_ascii() {
                self.consume_remnants_of_bad_url();
                return Token::bad_url(buf, location);
            } else if Self::is_valid_escape(self.stream.peek::<0, 2>()) {
                buf.push(self.stream.next().unwrap());
            } else if c == '\\' {
                self.consume_remnants_of_bad_url();
                return Token::bad_url(buf, location);
            } else {
                buf.push(c);
            }
        }

        Token::bad_url(buf, location)
    }

    /// Consume the next code-points and returns either an <ident-token>, a <func-token>, or a <url-token>.
    ///
    /// It expects the stream's next code point to be the first code point of
    /// an ident sequence.
    fn consume_ident_token(&mut self) -> Token {
        let location = self.stream.current_location.shift_right();

        let seq = self.consume_ident_sequence();

        if self.stream.peek::<1, 1>() == "(" {
            if seq == "url" {
                self.stream.next();
                self.consume_whitespace();

                if self
                    .stream
                    .current()
                    .map(|c| c == '\'' || c == '"')
                    .unwrap_or(false)
                {
                    return Token::function(seq, location);
                }

                return self.consume_url_token(location);
            }

            return Token::function(seq, location);
        }

        Token::ident(seq, location)
    }

    #[inline]
    fn is_whitespace_code_point(c: char) -> bool {
        c.is_ascii_whitespace()
    }

    #[inline]
    fn is_decimal_part_start(seq: &str) -> bool {
        seq.chars().nth(0).map(|c| c == '.').unwrap_or(false)
            && seq
                .chars()
                .nth(1)
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
    }
    #[inline]
    fn is_exponent_part_start(seq: &str) -> bool {
        seq.chars()
            .nth(0)
            .map(|c| c == 'e' || c == 'E')
            .unwrap_or(false)
            && seq
                .chars()
                .nth(1)
                .map(|c| c == '+' || c == '-')
                .unwrap_or(false)
            && seq
                .chars()
                .nth(2)
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
    }

    #[inline]
    fn is_digit_code_point(c: char) -> bool {
        c.is_ascii_digit()
    }
    #[inline]
    /// Checks if the code point is valid as an indent-token start.
    fn is_ident_start_code_point(c: char) -> bool {
        c.is_ascii_alphabetic() || c.is_ascii_control() || c == '_'
    }

    #[inline]
    fn is_ident_code_point(c: char) -> bool {
        c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_' || c == '-'
    }

    #[inline]
    fn is_valid_escape(seq: &str) -> bool {
        seq.chars().nth(0).map(|c| c == '\\').unwrap_or(false)
            && seq.chars().nth(1).map(|c| c != '\n').unwrap_or(true)
    }

    #[inline]
    fn is_number_token_start(seq: &str) -> bool {
        if &seq[0..1] == "+" || &seq[0..1] == "-" {
            if seq
                .chars()
                .nth(1)
                .map(Self::is_digit_code_point)
                .unwrap_or(false)
            {
                return true;
            }

            if seq.chars().nth(1).map(|c| c == '.').unwrap_or(false)
                && seq
                    .chars()
                    .nth(2)
                    .map(Self::is_digit_code_point)
                    .unwrap_or(false)
            {
                return true;
            }

            return false;
        }

        if seq.chars().nth(0).map(|c| c == '.').unwrap_or(false)
            && seq
                .chars()
                .nth(1)
                .map(Self::is_digit_code_point)
                .unwrap_or(false)
        {
            return true;
        }
        if seq
            .chars()
            .nth(0)
            .map(Self::is_digit_code_point)
            .unwrap_or(false)
        {
            return true;
        }

        false
    }

    /// Check if the sequence is a CDC (-->)
    ///
    /// This method requires three code points to match -->.
    #[inline]
    fn is_cdc_sequence(seq: &str) -> bool {
        seq.chars().nth(0).map(|c| c == '-').unwrap_or(false)
            && seq.chars().nth(1).map(|c| c == '-').unwrap_or(false)
            && seq.chars().nth(2).map(|c| c == '>').unwrap_or(false)
    }

    #[inline]
    fn is_cdo_sequence(seq: &str) -> bool {
        seq.chars().nth(0).map(|c| c == '<').unwrap_or(false)
            && seq.chars().nth(1).map(|c| c == '-').unwrap_or(false)
            && seq.chars().nth(2).map(|c| c == '-').unwrap_or(false)
    }
    /// Checks up to 3 code points to check if it's the start of an ident token.
    fn is_ident_sequence_start(seq: &str) -> bool {
        if seq
            .chars()
            .nth(0)
            .map(Self::is_ident_start_code_point)
            .unwrap_or(false)
        {
            return true;
        }

        if seq.chars().nth(0).map(|c| c == '-').unwrap_or(false) {
            if seq
                .chars()
                .nth(1)
                .map(Self::is_ident_start_code_point)
                .unwrap_or(false)
            {
                return true;
            }

            if seq.chars().nth(1).map(|c| c == '-').unwrap_or(false) {
                return true;
            }

            if seq.chars().nth(1).map(|c| c == '\\').unwrap_or(false)
                && Self::is_valid_escape(&seq[1..=2])
            {
                return true;
            }
        }

        if seq.chars().nth(0).map(|c| c == '\\').unwrap_or(false)
            && Self::is_valid_escape(&seq[1..3])
        {
            return true;
        }

        false
    }

    fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.stream.next() {
            let location = self.stream.current_location;

            // Consume comments
            if c == '/' && self.stream.peek::<0, 2>() == "/*" {
                self.consume_comment();
                continue;
            }

            // Consume whitespaces
            if Self::is_whitespace_code_point(c) {
                self.consume_whitespace();
                return Some(Token::whitespace(location));
            }

            // Consume an ident-token
            if Self::is_ident_start_code_point(c) {
                self.stream.rewind();
                return Some(self.consume_ident_token());
            }

            // Consume a string-token
            if c == '"' || c == '\'' {
                self.stream.rewind();
                return Some(self.consume_string_token());
            }

            // Consumes either an at-keyword, or a delim.
            if c == '@' {
                if Self::is_ident_sequence_start(self.stream.peek::<1, 3>()) {
                    return Some(Token::at_keyword(self.consume_ident_sequence(), location));
                }

                return Some(Token::delim(c, location));
            }
            // Consume either a delim, or a hash-token.
            if c == '#' {
                if Self::is_ident_sequence_start(self.stream.peek::<1, 3>()) {
                    let value = self.consume_ident_sequence();
                    return Some(Token::hash(value, location));
                }

                return Some(Token::delim(c.to_string(), location));
            }

            if c == '\\' {
                if Self::is_valid_escape(self.stream.peek::<0, 3>()) {
                    self.stream.rewind();
                    return Some(Token::ident(self.consume_ident_sequence(), location));
                }

                return Some(Token::delim(c, location));
            }
            if c == '(' {
                return Some(Token::opening_parenthesis(location));
            }

            if c == ')' {
                return Some(Token::closing_parenthesis(location));
            }

            if c == '+' || c == '-' {
                if Self::is_number_token_start(self.stream.peek::<0, 3>()) {
                    self.stream.rewind();
                    return Some(self.consume_number_token());
                }

                // Match CDC sequence -->
                if c == '-' && Self::is_cdc_sequence(self.stream.peek::<0, 3>()) {
                    self.stream.next();
                    self.stream.next();
                    return Some(Token::cdc(location));
                }

                return Some(Token::delim(c, location));
            }

            if Self::is_digit_code_point(c) {
                self.stream.rewind();
                return Some(self.consume_number_token());
            }

            if c == '.' {
                if Self::is_number_token_start(self.stream.peek::<0, 3>()) {
                    self.stream.rewind();
                    return Some(self.consume_number_token());
                }

                return Some(Token::delim(c, location));
            }

            if c == '<' {
                if Self::is_cdo_sequence(self.stream.peek::<0, 3>()) {
                    self.stream.next();
                    self.stream.next();
                    return Some(Token::cdo(location));
                }

                return Some(Token::delim(c, location));
            }

            if c == '{' {
                return Some(Token::opening_curly_bracket(location));
            }

            if c == '}' {
                return Some(Token::closing_curly_bracket(location));
            }

            if c == '[' {
                return Some(Token::opening_square_bracket(location));
            }

            if c == ']' {
                return Some(Token::closing_square_bracket(location));
            }

            if c == ',' {
                return Some(Token::comma(location));
            }

            if c == ':' {
                return Some(Token::colon(location));
            }

            if c == ';' {
                return Some(Token::semicolon(location));
            }

            return Some(Token::delim(c, location));
        }

        None
    }

    pub fn rewind(&mut self) {}
}

impl<'i> traits::Lexer<Token> for Lexer<'i> {
    fn current(&self) -> Option<Self::Item> {
        self.current.clone()
    }

    fn rewind(&mut self) {
        self.buffer.push(self.current.clone().unwrap());
    }
}

impl<'i> Iterator for Lexer<'i> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buffer.is_empty() {
            self.current = self.buffer.pop();
        } else {
            self.current = self.next_token();
        }

        self.current.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::style::lexer::Number;

    use super::{Lexer, SourceLocation, Token};

    #[test]
    fn test_001_string_token() {
        let mut lexer = Lexer::new("\"this is a string !\"");
        let token = lexer.next().unwrap();

        let expected_token =
            Token::string("this is a string !".to_string(), SourceLocation::new(1, 1));

        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_001_01_after_string_token_consumption() {
        let lexer = Lexer::new("\"this is a string\";");
        let tokens = lexer.collect::<Vec<_>>();
        let expected_tokens = vec![
            Token::string("this is a string", SourceLocation::new(1, 1)),
            Token::semicolon(SourceLocation::new(1, 19)),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_002_string_token_with_valid_escape() {
        let mut lexer = Lexer::new("\"this is a \\\"string\\\" !\"");
        let token = lexer.next().unwrap();

        let expected_token = Token::string(
            "this is a \"string\" !".to_string(),
            SourceLocation::new(1, 1),
        );

        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_003_string_token_bad_string() {
        let mut lexer = Lexer::new(
            "\"this is a bad string
            ",
        );
        let token = lexer.next().unwrap();

        let expected_token = Token::bad_string("this is a bad string", SourceLocation::new(1, 1));

        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_004_hash_token() {
        let mut lexer = Lexer::new("#ident_token");
        let token = lexer.next().unwrap();
        let expected_token = Token::hash("ident_token", SourceLocation::new(1, 1));
        assert_eq!(token, expected_token)
    }

    #[test]
    fn test_005_delim_hash() {
        let mut lexer = Lexer::new("#");
        let token = lexer.next().unwrap();
        let expected_token = Token::delim('#', SourceLocation::new(1, 1));
        assert_eq!(token, expected_token)
    }

    #[test]
    fn test_006_01_consume_digits() {
        let mut lexer = Lexer::new("12345p");
        let digits = lexer.consume_digits();
        assert_eq!(digits, "12345");
        assert_eq!(lexer.stream.next(), Some('p'));
    }

    #[test]
    fn test_006_number_token() {
        let mut lexer = Lexer::new("+101");
        let mut token = lexer.next().unwrap();
        let mut expected_token = Token::number("+101", SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);

        lexer = Lexer::new("-101");
        token = lexer.next().unwrap();
        expected_token = Token::number("-101", SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);

        lexer = Lexer::new("101");
        token = lexer.next().unwrap();
        expected_token = Token::number("101", SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);

        lexer = Lexer::new("123.456");
        token = lexer.next().unwrap();
        expected_token = Token::number(Number::new("123", "456", ""), SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);

        lexer = Lexer::new(".456");
        token = lexer.next().unwrap();
        expected_token = Token::number(Number::new("", "456", ""), SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);

        lexer = Lexer::new("-123.456e+789");
        token = lexer.next().unwrap();
        expected_token = Token::number(
            Number::new("-123", "456", "+789"),
            SourceLocation::new(1, 1),
        );
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_007_dimension_token() {
        let mut lexer = Lexer::new("123.56px");
        let token = lexer.next().unwrap();
        let expected_token = Token::dimension("123.56", "px", SourceLocation::new(1, 1));
        assert_eq!(token, expected_token)
    }

    #[test]
    fn test_008_percentage_token() {
        let mut lexer = Lexer::new("69%");
        let token = lexer.next().unwrap();
        let expected_token = Token::percentage("69", SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_009_opening_parenthesis() {
        let mut lexer = Lexer::new("(");
        let token = lexer.next().unwrap();
        let expected_token = Token::opening_parenthesis(SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_010_closing_parenthesis() {
        let mut lexer = Lexer::new(")");
        let token = lexer.next().unwrap();
        let expected_token = Token::closing_parenthesis(SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_011_opening_curly_bracket() {
        let mut lexer = Lexer::new("{");
        let token = lexer.next().unwrap();
        let expected_token = Token::opening_curly_bracket(SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_012_closing_curly_bracket() {
        let mut lexer = Lexer::new("}");
        let token = lexer.next().unwrap();
        let expected_token = Token::closing_curly_bracket(SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_013_opening_square_bracket() {
        let mut lexer = Lexer::new("[");
        let token = lexer.next().unwrap();
        let expected_token = Token::opening_square_bracket(SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_014_clsoing_square_bracket() {
        let mut lexer = Lexer::new("]");
        let token = lexer.next().unwrap();
        let expected_token = Token::closing_square_bracket(SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_015_comma() {
        let mut lexer = Lexer::new(",");
        let token = lexer.next().unwrap();
        let expected_token = Token::comma(SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_016_cdc() {
        let mut lexer = Lexer::new("-->");
        let token = lexer.next().unwrap();
        let expected_token = Token::cdc(SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_017_colon() {
        let mut lexer = Lexer::new(":");
        let token = lexer.next().unwrap();
        let expected_token = Token::colon(SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_018_cdo() {
        let mut lexer = Lexer::new("<--");
        let token = lexer.next().unwrap();
        let expected_token = Token::cdo(SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_019_semicolon() {
        let mut lexer = Lexer::new(";");
        let token = lexer.next().unwrap();
        let expected_token = Token::semicolon(SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_020_ident() {
        let mut lexer = Lexer::new("background-repeat");
        let token = lexer.next().unwrap();
        let expected_token = Token::ident("background-repeat", SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_021_skip_comments() {
        let mut lexer = Lexer::new("/* Comment */");
        assert!(lexer.next().is_none())
    }

    #[test]
    fn test_022_at_keyword() {
        let mut lexer = Lexer::new("@keyword");
        let token = lexer.next().unwrap();
        let expected_token = Token::at_keyword("keyword", SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_023_function_token() {
        let mut lexer = Lexer::new("func(");
        let token = lexer.next().unwrap();
        let expected_token = Token::function("func", SourceLocation::new(1, 1));
        assert_eq!(token, expected_token)
    }

    #[test]
    fn test_024_url_token() {
        let mut lexer = Lexer::new("url(http://url.lan)");
        let token = lexer.next().unwrap();
        let expected_token = Token::url("http://url.lan", SourceLocation::new(1, 1));
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_025_semicolon() {
        let mut lexer = Lexer::new(";");
        let token = lexer.next().unwrap();
        let expected_token = Token::semicolon(SourceLocation::new(1, 1));
        assert_eq!(token, expected_token)
    }

    #[test]
    fn test_100_complex_sequence() {
        let lexer = Lexer::new("background-repeat @at-keyword 123.45pt func(test, 10)");
        let tokens: Vec<Token> = lexer.collect::<Vec<_>>();
        let expected_tokens = vec![
            Token::ident("background-repeat", SourceLocation::new(1, 1)),
            Token::whitespace(SourceLocation::new(1, 18)),
            Token::at_keyword("at-keyword", SourceLocation::new(1, 19)),
            Token::whitespace(SourceLocation::new(1, 30)),
            Token::dimension("123.45", "pt", SourceLocation::new(1, 31)),
            Token::whitespace(SourceLocation::new(1, 39)),
            Token::function("func", SourceLocation::new(1, 40)),
            Token::opening_parenthesis(SourceLocation::new(1, 44)),
            Token::ident("test", SourceLocation::new(1, 45)),
            Token::comma(SourceLocation::new(1, 49)),
            Token::whitespace(SourceLocation::new(1, 50)),
            Token::number("10", SourceLocation::new(1, 51)),
            Token::closing_parenthesis(SourceLocation::new(1, 53)),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_101() {
        let lexer = Lexer::new("@charset \"utf-8\";");
        let tokens = lexer.collect::<Vec<_>>();
        let expected_tokens = vec![
            Token::at_keyword("charset", SourceLocation::new(1, 1)),
            Token::whitespace(SourceLocation::new(1, 9)),
            Token::string("utf-8", SourceLocation::new(1, 10)),
            Token::semicolon(SourceLocation::new(1, 17)),
        ];
        assert_eq!(tokens, expected_tokens)
    }

    #[test]
    fn test_102_multilines() {
        let lexer = Lexer::new(
            "keyword-1

        keyword-2
        ",
        );

        let tokens = lexer.collect::<Vec<_>>();
        let expected_tokens = vec![
            Token::ident("keyword-1", SourceLocation::new(1, 1)),
            Token::whitespace(SourceLocation::new(1, 10)),
            Token::ident("keyword-2", SourceLocation::new(3, 8)),
            Token::whitespace(SourceLocation::new(3, 17)),
        ];

        assert_eq!(tokens, expected_tokens);
    }
}
