use std::ops::{Deref, DerefMut};

use super::{fragment::WeakFragment, Edge};
use crate::{collections::tree, document, geometry::Block, style::{self, Style}};

/// Définit la taille d'une page de noeuds.
const BOX_TREE_PAGE_SIZE: usize = 50;

type InnerTree<Unit> = tree::Tree<BOX_TREE_PAGE_SIZE, Box<Unit>>;

/// Un noeud de l'arbre à boîtes.
pub type BoxTreeNode<Unit>      = tree::TreeNode<BOX_TREE_PAGE_SIZE, Box<Unit>>;
pub type WeakBoxTreeNode<Unit>  = tree::WeakTreeNode<BOX_TREE_PAGE_SIZE, Box<Unit>>;
pub type RefBoxTreeNode<Unit>   = tree::RefTreeNode<BOX_TREE_PAGE_SIZE, Box<Unit>>;
pub type MutBoxTreeNode<Unit>   = tree::MutTreeNode<BOX_TREE_PAGE_SIZE, Box<Unit>>;

#[derive(Default)]
pub struct BoxTree<Unit>(InnerTree<Unit>) where Unit: 'static;

impl<Unit> Deref for BoxTree<Unit> {
    type Target = InnerTree<Unit>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Unit> DerefMut for BoxTree<Unit> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
    pub element: Option<document::WeakDocumentNode>,
    
    /// Style properties
    pub style: style::Style,

    /// Containeur primordial du sous-arbre de fragments lié à la boîte.
    pub fragment: WeakFragment<Unit>
}
