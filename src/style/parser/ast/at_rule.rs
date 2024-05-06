use cssparser::{Parser, Token};

use crate::style::{consume, peek, ParseResult};

use super::{SimpleBlock, ComponentValue};

#[derive(Default)]
pub struct AtRule<'i> {
    pub prelude: Vec<ComponentValue<'i>>,
    pub block: SimpleBlock<'i>,
}


impl<'i> AtRule<'i> {
    /// Consumes an At-Rule from the lexer.
    pub fn consume(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
        let mut at_rule = Self::default();

        loop {
            match peek(parser)? {
                Token::Semicolon => {
                    consume(parser)?;
                    return Ok(at_rule);
                }
                Token::CurlyBracketBlock => {
                    consume(parser)?;
                    at_rule.block = parser.parse_nested_block(SimpleBlock::consume)?;
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