pub mod ast;
pub mod lexer;

use std::vec;

use cssparser::{ParseError, SourceLocation, Token};
use self::ast::{ComponentValue, Rule};

pub type ParseResult<'i, T> = Result<T, ParseError<'i, CustomParseError>>;

pub enum CustomParseError {
    Expecting(&'static str)
}

/// Universal parser
enum ParserCore<'a, 'i, 't> {
    Base(&'a mut cssparser::Parser<'i, 't>),
    Iterator(Box<dyn Iterator<Item=ComponentValue<'i>>>)
}

pub struct Parser<'a, 'i, 't> {
    core: ParserCore<'a, 'i, 't>,
    current_source_location: SourceLocation,
    buffer: Vec<ComponentValue<'i>>
}

impl<'a, 'i, 't> Parser<'a, 'i, 't> {
    pub fn new_from_base(base: &'a mut cssparser::Parser<'i, 't>) -> Self {
        Self {
            core: ParserCore::Base(base),
            current_source_location: base.current_source_location(),
            buffer: vec![]
        }
    }

    pub fn new<I: Iterator<Item=ComponentValue<'i>>>(start: SourceLocation, it: I) -> Self {
        Self {
            core: ParserCore::Iterator(it),
            current_source_location: start,
            buffer: vec![]
        }
    }
}

impl<'a, 'i, 't> Iterator for Parser<'a, 'i, 't> {
    type Item = ParseResult<'i, ComponentValue<'i>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cv) = self.buffer.pop() {
            return Some(Ok(cv))
        }
        match self.core {
            ParserCore::Base(base) => {
                if base.is_exhausted() {
                    return None
                }
                let token = base.next()?.clone();
                self.current_source_location = base.current_source_location();
                
                Some(Ok(ComponentValue::PreservedToken { location: self.current_source_location.clone(), token }))
            },

            ParserCore::Iterator(mut iter) => {
                let cv = iter.next()?;
                self.current_source_location = cv.location().clone();
                
                Some(Ok(cv))
            },
        }
    }
}

impl<'a, 'i, 't> CssParser<'i> for Parser<'a, 'i, 't> {
    fn current_source_location(&self) -> SourceLocation {
        self.current_source_location
    }

    fn peek(&mut self) -> Self::Item {
        let cv = self.next()?;
        self.buffer.push(cv);
        Ok(cv)
    }

}

pub trait CssParser<'i>: Iterator<Item=ParseResult<'i, ComponentValue<'i>>> {
    /// Return the current location in the stream.
    fn current_source_location(&self) -> SourceLocation;
    
    /// Peek the next component value
    fn peek(&mut self) -> Self::Item;

    /// Peek a token
    fn peek_token(&mut self) -> ParseResult<'i, Token<'i>>
    {
        self
            .next()
            .ok_or_else(self.current_source_location().new_basic_error(cssparser::BasicParseErrorKind::EndOfInput))
            .flatten()?
    }

    /// Consume the next token
    fn next_token(&mut self) -> ParseResult<'i, Token<'i>> {
        Some(self.next()?.try_into_token())
    }
}

pub fn parse<'i>(content: &'i str) -> Result<ast::Sheet<'i>, ParseError<'i, Error>> {
    let mut ipt = cssparser::ParserInput::new(content);
    let mut parser = cssparser::Parser::new(&mut ipt);

    let mut sheet = ast::Sheet::default();

    while !parser.is_exhausted() {
        sheet.add_rule(parser.try_parse(Rule::consume)?);
    }

    Ok(sheet)
}
