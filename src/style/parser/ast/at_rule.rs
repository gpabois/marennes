use std::default;

use cssparser::{Parser, SourceLocation, Token};

use crate::style::{consume, peek, ParseResult};

use super::{ComponentValue, Declarations, SimpleBlock};

pub struct AtRule<'i> {
    pub location: SourceLocation, 
    pub prelude: Vec<ComponentValue<'i>>,
    pub block: SimpleBlock<'i>,
}

impl<'i> AtRule<'i> {
    /// Consumes an At-Rule from the lexer.
    pub fn consume(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
        let location = parser.current_source_location();
        let mut prelude = Vec<ComponentValue<'i>>::default();

        loop {
            match peek(parser)? {
                Token::Semicolon => {
                    consume(parser)?;
                    return Ok(at_rule);
                }
                Token::CurlyBracketBlock => {
                    consume(parser)?;
                    at_rule.declarations = SimpleBlock::consume(parser);
                    return Ok(at_rule);
                }
                _ => {
                    let component_value = ComponentValue::consume(parser)?;
                    at_rule.prelude.push(component_value);
                }
            }
        }
    }
}