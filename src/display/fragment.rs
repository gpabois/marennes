use std::ops::{Deref, DerefMut};

use super::TextSequence;
use crate::collections::tree;

pub type FragmentId = tree::TreeNodeId;

/// A box tree fragment
pub enum Fragment<Unit> {
    /// The fragment is directly the primordial box.
    Box(Box<Unit>),

    /// Séquence de texte
    Text(TextSequence),
}

impl<Unit> Fragment<Unit> {
    pub fn new_text_sequence(text: &str) -> Self {
        Self::Text(TextSequence::new(text))
    }
}

pub struct FragmentTree<Unit>(tree::Tree<50, Fragment<Unit>>)
where
    Unit: 'static;

impl<Unit> FragmentTree<Unit> {
    pub fn new_text_sequence<S: AsRef<str>>(&mut self, text: S) -> FragmentId {
        self.new_node(Fragment::new_text_sequence(text.as_ref()))
    }
}
impl<Unit> Default for FragmentTree<Unit> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<Unit> Deref for FragmentTree<Unit> {
    type Target = tree::Tree<50, Fragment<Unit>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Unit> DerefMut for FragmentTree<Unit> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
