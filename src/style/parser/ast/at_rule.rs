use super::{Block, ComponentValue};

#[derive(Default)]
pub struct AtRule<'i> {
    pub prelude: Vec<ComponentValue<'i>>,
    pub block: Option<Block>,
}
