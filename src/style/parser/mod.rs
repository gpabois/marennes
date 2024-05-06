pub mod ast;

use std::path::Iter;

use cssparser::{BasicParseError, ParseError, Parser, SourceLocation, Token};

use crate::iter::{SplitIterator, Splittable};

use self::ast::{ComponentValue, Declaration, Declarations, QualifiedRule, Rule, SimpleBlock};

use super::{Dimension, Number, Value};

pub type ParseResult<'i, T> = Result<T, ParseError<'i, Error>>;

pub enum Error {}

pub fn consume<'i>(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Token<'i>> {
    let tok = parser.next()?.clone();
    Ok(tok)
}

pub fn peek<'i>(
    parser: &mut Parser<'i, '_>,
) -> Result<Token<'i>, ParseError<'i, Error>> {
    let state = parser.state();
    let tok = parser.next()?.clone();
    parser.reset(&state);
    Ok(tok)
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
