use std::ops::{Deref, DerefMut};

use super::{r#box::BoxTreeNodeId, TextSequence};
use crate::collections::tree;

pub type FragmentId = tree::TreeNodeId;

/// A box tree fragment
pub enum Fragment<Unit> {
    /// The fragment is directly the primordial box.
    Box(BoxTreeNodeId),

    /// Séquence de texte
    Text(BoxTreeNodeId),
}

pub struct FragmentTree<Unit>(tree::Tree<50, Fragment<Unit>>)
where
    Unit: 'static;


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
