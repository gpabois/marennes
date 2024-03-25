use std::ops::{Deref, DerefMut};

use crate::{collections::tree, style::Style};

/// Définit la taille d'une page de noeuds.
pub const DOCUMENT_PAGE_SIZE: usize = 50;

type InnerTree = tree::Tree<DOCUMENT_PAGE_SIZE, DocumentNodeValue>;

/// Un noeud d'un document.
pub type DocumentNode = tree::TreeNode<DOCUMENT_PAGE_SIZE, DocumentNodeValue>;
pub type WeakDocumentNode = tree::WeakTreeNode<DOCUMENT_PAGE_SIZE, DocumentNodeValue>;
pub type RefDocumentNode = tree::RefTreeNode<DOCUMENT_PAGE_SIZE, DocumentNodeValue>;
pub type MutDocumentNode = tree::MutTreeNode<DOCUMENT_PAGE_SIZE, DocumentNodeValue>;

/// An HTML document
#[derive(Default)]
pub struct Document(InnerTree);

impl Deref for Document {
    type Target = InnerTree;

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
pub enum DocumentNodeValue {
    Element(Element),
    Text(Text),
}

/// Element node
pub struct Element {
    pub style: Style,
}

/// Text node
pub struct Text{ 
    pub style: Style,
    pub text: String
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

