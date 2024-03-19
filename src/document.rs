use std::ops::{Deref, DerefMut};

use crate::{collections::tree, style::Style};

pub type NodeId = tree::TreeNodeId;

/// An HTML document
#[derive(Default)]
pub struct Document(tree::Tree<50, DocumentNode>);

impl Deref for Document {
    type Target = tree::Tree<50, DocumentNode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Document {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Document's node
pub enum DocumentNode {
    Element(Element),
    Text(Text),
}

/// Element node
pub struct Element {
    pub style: Style,
}

/// Text node
pub struct Text(String);

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
