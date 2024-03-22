use std::ops::{Deref, DerefMut};

use super::{
    book::{BookItemId, BookResult, MutBookEntry, RefBookEntry},
    Book,
};

pub type TreeNodeId = BookItemId;
pub type TreeResult<D> = BookResult<D>;
pub type RefTreeNode<'a, Data> = RefBookEntry<'a, TreeNode<Data>>;
pub type MutTreeNode<'a, Data> = MutBookEntry<'a, TreeNode<Data>>;

pub struct TreeNode<Data> {
    id: TreeNodeId,
    data: Data,
    pub parent: Option<TreeNodeId>,
    pub children: Vec<TreeNodeId>,
}

impl<Data> Deref for TreeNode<Data> {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<Data> DerefMut for TreeNode<Data> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<Data> TreeNode<Data> {
    /// Récupère l'identifiant du noeud.
    pub fn id(&self) -> &TreeNodeId {
        &self.id
    }
}

/// Un catalogue est un livre où les entrées sont auto-référencées
/// (ie ils connaissent leurs identifiants)
///
/// N correspond au nombre de noeuds contenus dans une page contiguë.
pub struct Tree<const N: usize, Data>
where
    Data: 'static,
{
    root: Option<TreeNodeId>,
    nodes: Book<N, TreeNode<Data>>,
}
impl<const N: usize, Data> Default for Tree<N, Data> {
    fn default() -> Self {
        Self {
            root: None,
            nodes: Book::default(),
        }
    }
}

impl<const N: usize, Data> Tree<N, Data> {
    pub fn root(&self) -> Option<&TreeNodeId> {
        self.root.as_ref()
    }

    pub fn set_root(&mut self, id: &TreeNodeId) {
        self.root = Some(id.clone())
    }
    /// Crée un nouveau noeud.
    ///
    /// Cette fonction ne crée pas de liens entre les noeuds (Parent/enfants)
    pub fn new_node<D: Into<Data>>(&mut self, data: D) -> TreeNodeId {
        unsafe {
            let id = self.nodes.alloc_entry();
            let node = TreeNode {
                id: id.clone(),
                data: data.into(),
                parent: None,
                children: Vec::default(),
            };

            self.nodes.init_entry(&id, node);
            id
        }
    }

    /// Emprunte un noeud, s'il existe, et pas déjà mut-emprunté.
    pub fn try_borrow<'a>(&'a self, id: &TreeNodeId) -> Option<TreeResult<RefTreeNode<'a, Data>>> {
        self.nodes.try_get(id)
    }

    /// Emprunte un noeud, panique s'il n'existe pas, ou est déjà mut-emprunté.
    pub fn borrow<'a>(&'a self, id: &TreeNodeId) -> RefTreeNode<'a, Data> {
        self.nodes.get(id)
    }

    /// Mut-emprunte un noeud, s'il existe, et pas déjà emprunté ou mut-emprunté.
    pub fn try_borrow_mut<'a>(
        &'a self,
        id: &TreeNodeId,
    ) -> Option<TreeResult<MutTreeNode<'a, Data>>> {
        self.nodes.try_get_mut(id)
    }

    pub fn borrow_mut<'a>(&'a self, id: &TreeNodeId) -> MutTreeNode<'a, Data> {
        self.nodes.get_mut(id)
    }
}
