use crate::style::{
    traits::{Lexer, Parser},
    Token, TokenKind,
};

use super::{Function, SimpleBlock};

#[derive(Debug, PartialEq)]
pub enum ComponentValue {
    Block(SimpleBlock),
    Function(Function),
    Token(Token),
}

impl ComponentValue {
    pub fn token(token: Token) -> Self {
        Self::Token(token)
    }

    pub fn if_token<F: Fn(&Token) -> R, R>(&self, func: F) -> Option<R> {
        match self {
            Self::Token(tok) => Some(func(tok)),
            _ => None,
        }
    }
}

impl From<Token> for ComponentValue {
    fn from(value: Token) -> Self {
        Self::token(value)
    }
}

impl Parser<Token> for ComponentValue {
    fn parse<L: Lexer<Token>>(lexer: &mut L) -> Self {
        let token = lexer.current().unwrap();

        if matches!(
            token.kind,
            TokenKind::OpeningSquareBracket
                | TokenKind::OpeningParenthesis
                | TokenKind::OpeningCurlyBracket
        ) {
            lexer.rewind();
            ComponentValue::Block(SimpleBlock::parse(lexer))
        } else if matches!(token.kind, TokenKind::Function(_)) {
            lexer.rewind();
            ComponentValue::Function(Function::parse(lexer))
        } else {
            ComponentValue::Token(token)
        }
    }
}
