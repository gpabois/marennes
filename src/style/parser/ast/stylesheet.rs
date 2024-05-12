use crate::style::{
    traits::{Lexer, Parser},
    Token,
};

use super::Rules;

pub struct Stylesheet {
    pub rules: Rules,
}

impl Parser<Token> for Stylesheet {
    fn parse<L: Lexer<Token>>(lexer: &mut L) -> Self {
        Self {
            rules: Rules::parse(lexer),
        }
    }
}
