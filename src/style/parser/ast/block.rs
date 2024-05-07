use cssparser::{Parser, SourceLocation};

use crate::style::ParseResult;

use super::ComponentValue;

#[derive(Clone)]
pub struct SimpleBlock<'i> {
    pub location: SourceLocation,
    values: Vec<ComponentValue<'i>>
}

impl SimpleBlock<'_> {
    pub fn new(location: SourceLocation) -> Self {
        Self {
            location,
            values: vec![]
        }
    }
}

impl<'i> IntoIterator for SimpleBlock<'i> {
    type Item = ComponentValue<'i>;
    type IntoIter = <Vec<ComponentValue<'i>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'i> SimpleBlock<'i> {
    /// Consumes a simple-block
    pub fn consume(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
        parser.parse_nested_block(|parser| {
            let mut sblk = Self::new(parser.current_source_location());

            while !parser.is_exhausted() {
                sblk.push(ComponentValue::consume(parser)?);
            }
            
            Ok(sblk)
        })
    }
}

impl<'i> SimpleBlock<'i> {
    /// Push a new component value in the block.
    pub fn push(&mut self, value: ComponentValue<'i>) {
        self.values.push(value)
    }

    /// Iterate over all the component values within the block.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item=&'a ComponentValue<'i>> + 'a {
        self.values.iter()
    }
}
