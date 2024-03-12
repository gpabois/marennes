use std::collections::BTreeMap;

use crate::style::Style;

pub type NodeId = usize;

/// An HTML document
#[derive(Default)]
pub struct Document {
    counter: usize,
    inner: BTreeMap<NodeId, Node>
}

impl Document {
    /// Allocate a new document node
    pub fn alloc(&mut self, node: Node) -> NodeId {
        self.counter += 1;
        let id = self.counter;
        self.inner.insert(id, node);
        id
    }

    pub fn get(&self, id: &NodeId) -> Option<&Node> {
        self.inner.get(id)
    }

    pub fn get_mut(&mut self, id: &NodeId) -> Option<&mut Node> {
        self.inner.get_mut(id)
    }
}

/// Document's node
pub enum Node {
    Element(Element),
    Text(Text)
}

/// Element node
pub struct Element {
    pub style: Style
}

/// Text node
pub struct Text;