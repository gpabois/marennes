use std::ops::{Deref, DerefMut};

use super::{Edge, TextSequence};
use crate::{collections::tree, document, geometry::Block, style::{self, Style}};

pub type BoxTreeNodeId = tree::TreeNodeId;

#[derive(Default)]
pub struct BoxTree<Unit>(tree::Tree<50, BoxTreeNode<Unit>>) where Unit: 'static;

impl<Unit> BoxTree<Unit> {
    pub fn new_text_sequence(&mut self, text: &str, style: Style) -> BoxTreeNodeId {
        self.new_node(TextSequence::new(text, style))
    }
}

impl<Unit> Deref for BoxTree<Unit> {
    type Target = tree::Tree<50, BoxTreeNode<Unit>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Unit> DerefMut for BoxTree<Unit> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub enum BoxTreeNode<Unit> {
    Box(Box<Unit>),
    Text(TextSequence)
}

impl<Unit> From<TextSequence> for BoxTreeNode<Unit> {
    fn from(value: TextSequence) -> Self {
        Self::Text(value)
    }
}

/// CSS Box Model
/// [Source](https://www.w3.org/TR/CSS22/box.html)
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
    pub element: Option<document::DocumentNodeId>,
    /// Style properties
    pub style: style::Style,
}
