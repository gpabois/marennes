use cssparser::{CowRcStr, Parser, SourceLocation, Token};

use crate::{iter::Splittable, style::ParseResult};

use super::ComponentValue;

#[derive(Clone)]
pub struct Function<'i> {
    pub name: CowRcStr<'i>,
    pub args: Vec<ComponentValue<'i>>,
    pub location: SourceLocation
}

impl<'i> Function<'i> {
    /// Consume a function
    pub fn consume(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
        let location = parser.current_source_location();
        let name = parser.expect_function()?.clone();
        let mut func = Self::new(name, location);
    
        parser.parse_nested_block::<_, Self, _>(move |parser| { 
            let mut args = Vec::<ComponentValue<'i>>::default();
            
            while !parser.is_exhausted() {
                args.push(ComponentValue::consume(parser)?);
            }
    
            func.args = args
                .into_iter()
                // split by ","
                .split_at(ComponentValue::is_comma)
                // we ensure a flatten list of args
                .flatten()
                // collect the args
                .collect();
    
            Ok(func)
        })
    }
}

impl<'i> Function<'i> {
    pub fn new(name: CowRcStr<'i>, location: SourceLocation) -> Self {
        Self {
            name,
            location,
            args: Vec::default(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
