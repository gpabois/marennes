use cssparser::{CowRcStr, Parser, Token};

use crate::style::ParseResult;

use super::ComponentValue;

pub struct Function<'i> {
    pub name: CowRcStr<'i>,
    pub args: Vec<ComponentValue<'i>>,
}

impl<'i> Function<'i> {
    /// Consume a function
    pub fn consume(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
        let name = parser.expect_function()?.clone();
        let mut func = Self::new(name);
    
        parser.parse_nested_block::<_, Self, _>(move |parser| { 
            let mut args = Vec::<ComponentValue<'i>>::default();
            
            while !parser.is_exhausted() {
                args.push(ComponentValue::consume(parser)?);
            }
    
            func.args = args
                .into_iter()
                // split by ","
                .split_at(|cv| ComponentValue::Token(Token::Comma))
                // we ensure a flatten list of args
                .flatten()
                // collect the args
                .collect();
    
            Ok(func)
        })
    }
}

impl<'i> Function<'i> {
    pub fn new(name: CowRcStr<'i>) -> Self {
        Self {
            name,
            args: Vec::default(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
