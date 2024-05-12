use crate::style::{
    traits::{Lexer, Parser},
    Token, TokenKind,
};

use super::{ComponentValue, SimpleBlock};

pub struct Rules(Vec<Rule>);

impl Parser<Token> for Rules {
    fn parse<L: Lexer<Token>>(lexer: &mut L) -> Self {
        let mut rules = Vec::<Rule>::default();

        while let Some(token) = lexer.next() {
            if matches!(token.kind, TokenKind::Whitespace) {
                continue;
            }

            if matches!(token.kind, TokenKind::CDO | TokenKind::CDC) {
                continue;
            }

            if matches!(token.kind, TokenKind::AtKeyword(_)) {
                lexer.rewind();
                rules.push(Rule::At(AtRule::parse(lexer)));
            } else {
                lexer.rewind();
                rules.push(Rule::Qualified(QualifiedRule::parse(lexer)));
            }
        }

        Self(rules)
    }
}

pub enum Rule {
    At(AtRule),
    Qualified(QualifiedRule),
}

#[derive(Default, Debug, PartialEq)]
pub struct AtRule {
    prelude: Vec<ComponentValue>,
    block: SimpleBlock,
}

impl AtRule {
    pub fn new<T, I>(prelude: I, block: SimpleBlock) -> Self
    where
        ComponentValue: From<T>,
        I: IntoIterator<Item = T>,
    {
        Self {
            prelude: prelude
                .into_iter()
                .map(ComponentValue::from)
                .collect::<Vec<_>>(),
            block,
        }
    }
}

impl Parser<Token> for AtRule {
    fn parse<L: Lexer<Token>>(lexer: &mut L) -> Self {
        let mut rule = Self::default();

        while let Some(token) = lexer.next() {
            if matches!(token.kind, TokenKind::Semicolon) {
                return rule;
            }

            if matches!(token.kind, TokenKind::OpeningCurlyBracket) {
                rule.block = SimpleBlock::parse(lexer);
            } else {
                rule.prelude.push(ComponentValue::parse(lexer));
            }
        }

        panic!("unexpected eof");
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct QualifiedRule {
    prelude: Vec<ComponentValue>,
    block: SimpleBlock,
}

impl QualifiedRule {
    pub fn new<T, I>(prelude: I, block: SimpleBlock) -> Self
    where
        ComponentValue: From<T>,
        I: IntoIterator<Item = T>,
    {
        Self {
            prelude: prelude.into_iter().map(ComponentValue::from).collect(),
            block,
        }
    }
}

impl Parser<Token> for QualifiedRule {
    fn parse<L: Lexer<Token>>(lexer: &mut L) -> Self {
        let mut rule = Self::default();

        while let Some(token) = lexer.next() {
            if matches!(token.kind, TokenKind::OpeningCurlyBracket) {
                rule.block = SimpleBlock::parse(lexer);
                return rule;
            } else {
                rule.prelude.push(ComponentValue::parse(lexer));
            }
        }

        panic!("unexpected eof");
    }
}
