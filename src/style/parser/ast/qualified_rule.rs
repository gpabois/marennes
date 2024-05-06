use cssparser::{Parser, Token};

use crate::style::{peek, ParseResult};

use super::{ComponentValue, Declarations, SimpleBlock};

#[derive(Default)]
pub struct QualifiedRule<'i> {
    pub prelude: Vec<ComponentValue<'i>>,
    pub declarations: Declarations<'i>
}

impl<'i> QualifiedRule<'i> {
    pub fn consume(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
        let mut rule = Self::default();
        let mut prelude: Vec<ComponentValue<'i>> = Vec::default();
    
        loop {
            match peek(parser)? {
                Token::CurlyBracketBlock => {
                    parser.next();
                    let decls: Declarations = SimpleBlock::consume(parser).flat_map(Declarations::try_from)?;
                    rule.declarations = decls;
                    return Ok(rule)
                },
                _ => {
                    let component_value = ComponentValue::consume(parser)?;
                    prelude.push(component_value);
                }
            }
        }
    }
}