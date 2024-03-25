use std::ops::{Deref, DerefMut};

use crate::collections::tree;

use super::r#box::WeakBoxTreeNode;

/// Définit la taille d'une page de noeuds.
const FRAGMENT_TREE_PAGE_SIZE: usize = 50;

type InnerTree<Unit> = tree::Tree<FRAGMENT_TREE_PAGE_SIZE, FragmentValue<Unit>>;

pub type Fragment<Unit>      = tree::TreeNode<FRAGMENT_TREE_PAGE_SIZE, FragmentValue<Unit>>;
pub type WeakFragment<Unit>  = tree::WeakTreeNode<FRAGMENT_TREE_PAGE_SIZE, FragmentValue<Unit>>;
pub type RefFragment<Unit>   = tree::RefTreeNode<FRAGMENT_TREE_PAGE_SIZE, FragmentValue<Unit>>;
pub type MutFragment<Unit>   = tree::MutTreeNode<FRAGMENT_TREE_PAGE_SIZE, FragmentValue<Unit>>;

pub struct FragmentTree<Unit>(InnerTree<Unit>)
where
    Unit: 'static;


impl<Unit> Default for FragmentTree<Unit> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<Unit> Deref for FragmentTree<Unit> {
    type Target = InnerTree<Unit>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Unit> DerefMut for FragmentTree<Unit> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A box tree fragment
pub enum FragmentValue<Unit> {
    /// The fragment is directly the primordial box.
    Box(WeakBoxTreeNode<Unit>),

    /// Séquence de texte
    Text(WeakBoxTreeNode<Unit>),
}
