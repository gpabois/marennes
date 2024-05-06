use cssparser::{Parser, Token};

use crate::style::{peek, ParseResult};

use super::{AtRule, QualifiedRule};

pub enum Rule<'i> {
    AtRule(AtRule<'i>),
    QualifiedRule(QualifiedRule<'i>),
}

impl<'i> Rule<'i> {
    pub fn consume(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
        match peek(parser)? {
            Token::AtKeyword(_) => AtRule::consume(parser).map(Self::from),
            _ => QualifiedRule::consume(parser).map(Self::from)
        }
    }
}

impl<'i> From<AtRule<'i>> for Rule<'i> {
    fn from(value: AtRule<'i>) -> Self {
        Self::AtRule(value)
    }
}

impl<'i> From<QualifiedRule<'i>> for Rule<'i> {
    fn from(value: QualifiedRule<'i>) -> Self {
        Self::QualifiedRule(value)
    }
}
