use std::collections::BTreeMap;

use super::{Edge, Level};
use crate::{dom, geometry::Block, style};

pub type BoxTreeNodeId = usize;
pub type BoxTreeNodeRef<'a, Unit> = &'a BoxTreeNode<Unit>;
pub type BoxTreeNodeMut<'a, Unit> = &'a mut BoxTreeNode<Unit>;

#[derive(Default)]
pub struct BoxTree<Unit> {
    counter: usize,
    inner: BTreeMap<NodeId, Node<Unit>>,
}

impl<Unit> BoxTree<Unit> {
    /// Allocate a new box in the tree
    pub fn alloc(&mut self, node: Node<Unit>) -> NodeId {
        self.counter += 1;
        let id = self.counter;
        self.inner.insert(id, node);
        id
    }

    /// Get a reference to a box tree node by its id.
    pub fn get(&self, id: &NodeId) -> Option<NodeRef<'_, Unit>> {
        self.inner.get(id)
    }

    /// Get a mutable reference to a box tree node by its id.
    pub fn get_mut(&mut self, id: &NodeId) -> Option<NodeMut<'_, Unit>> {
        self.inner.get_mut(id)
    }
}

/// A box tree node.
pub enum BoxTreeNode<Unit> {
    Box(Box<Unit>),
    TextSequence(TextSequence),
}

///
pub struct Box<Unit> {
    /// Containing block of the box
    /// Source: https://www.w3.org/TR/CSS22/visuren.html#containing-block
    pub content: Block<Unit>,
    /// The padding edge surrounds the box’s padding. If the padding has zero width on a given side, the padding edge coincides with the content edge on that side.
    /// The four sides of the padding edge together define the box’s padding box, which contains both the content and padding areas.
    /// Source: https://drafts.csswg.org/css-box-3/#padding-box
    pub padding: Edge<Unit>,
    /// The border edge surrounds the box’s border.
    /// Source: https://drafts.csswg.org/css-box-3/#border-box
    pub border: Edge<Unit>,
    /// The margin edge surrounds the box’s margin
    /// Source: https://drafts.csswg.org/css-box-3/#margin-box
    pub margin: Edge<Unit>,
    /// The element from which the box is generated
    /// Anonymous boxes has no element
    pub element: Option<dom::NodeId>,
    /// Style properties
    pub style: style::Style,
    /// Children of the box
    pub children: Vec<BoxTreeNodeId>,
    /// The formatting context of the box
    pub formatting_context: Level,
}

pub struct TextSequence {
    pub text: String,
    pub element: Option<dom::NodeId>,
    pub style: style::Style,
}

