use crate::style::{
    traits::{Lexer, Parser},
    Token, TokenKind,
};

use super::ComponentValue;

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub value: Vec<ComponentValue>,
}

impl Parser<Token> for Function {
    fn parse<L: Lexer<Token>>(lexer: &mut L) -> Self {
        let name = lexer.next().unwrap().kind.expect_ident().to_string();
        let mut value = Vec::<ComponentValue>::default();

        while let Some(token) = lexer.next() {
            if matches!(token.kind, TokenKind::ClosingParenthesis) {
                return Self { name, value };
            }

            value.push(ComponentValue::parse(lexer));
        }

        Self { name, value }
    }
}
