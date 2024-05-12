pub mod ast;
pub mod lexer;

pub use ast::*;
pub use lexer::*;

pub mod traits {
    pub use super::lexer::traits::Lexer;

    pub trait Parser<T> {
        fn parse<L: Lexer<T>>(lexer: &mut L) -> Self;
    }
}

#[cfg(test)]
mod test {
    use crate::style::{Lexer, SourceLocation, Token};

    use super::{
        ast::{AtRule, QualifiedRule, SimpleBlock},
        traits::Parser,
    };

    #[test]
    fn test_001_at_rule() {
        let mut lexer = Lexer::new("@charset \"utf-8\";");
        let rule = AtRule::parse(&mut lexer);
        let expected_rule = AtRule::new(
            [
                Token::at_keyword("charset", SourceLocation::new(1, 1)),
                Token::whitespace(SourceLocation::new(1, 9)),
                Token::string("utf-8", SourceLocation::new(1, 10)),
            ],
            SimpleBlock::default(),
        );

        assert_eq!(rule, expected_rule)
    }

    #[test]
    fn test_002_qualified_rule() {
        let mut lexer = Lexer::new(
            "p {
                color: red;
            }
        ",
        );

        let rule = QualifiedRule::parse(&mut lexer);
        let expected_rule = QualifiedRule::new(
            [
                Token::ident("p", SourceLocation::new(1, 1)),
                Token::whitespace(SourceLocation::new(1, 2)),
            ],
            SimpleBlock::new([
                Token::whitespace(SourceLocation::new(1, 4)),
                Token::ident("color", SourceLocation::new(2, 16)),
                Token::colon(SourceLocation::new(2, 21)),
                Token::whitespace(SourceLocation::new(2, 22)),
                Token::ident("red", SourceLocation::new(2, 23)),
                Token::semicolon(SourceLocation::new(2, 26)),
                Token::whitespace(SourceLocation::new(2, 27)),
            ]),
        );

        assert_eq!(rule, expected_rule)
    }
}
