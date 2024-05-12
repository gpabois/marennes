use crate::style::{
    traits::{Lexer, Parser},
    Token, TokenKind,
};

use super::ComponentValue;

#[derive(Default, Debug, PartialEq)]
pub struct SimpleBlock(Vec<ComponentValue>);

impl SimpleBlock {
    pub fn new<T, I>(values: I) -> Self
    where
        ComponentValue: From<T>,
        I: IntoIterator<Item = T>,
    {
        Self(values.into_iter().map(ComponentValue::from).collect())
    }
}

impl Parser<Token> for SimpleBlock {
    /// Parse a {}, [] or () block.
    ///
    /// It expects the lexer's current token to be the opening.
    fn parse<L: Lexer<Token>>(lexer: &mut L) -> Self {
        let del = lexer.current().unwrap();
        let mut block = SimpleBlock::default();

        while let Some(token) = lexer.next() {
            if matches!(
                (&del.kind, token.kind),
                (
                    TokenKind::OpeningCurlyBracket,
                    TokenKind::ClosingCurlyBracket
                ) | (TokenKind::OpeningParenthesis, TokenKind::ClosingParenthesis)
                    | (
                        TokenKind::OpeningSquareBracket,
                        TokenKind::ClosingSquareBracket
                    )
            ) {
                return block;
            } else {
                block.0.push(ComponentValue::parse(lexer));
            }
        }

        panic!("unexpected eof")
    }
}
