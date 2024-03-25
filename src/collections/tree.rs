use std::ops::{Deref, DerefMut};

use super::{book::BookError, Book, BookEntry, BookReadEntry, BookResult, BookWeakEntry};

pub type TreeError = BookError;
pub type TreeResult<D> = BookResult<D>;

/// Référence faible vers une noeud de l'arbre.
pub type WeakTreeNode<const N: usize, Data> = BookWeakEntry<N, TreeNode<N, Data>>;

/// Référence mutable sur un noeud de l'arbre.
pub type MutTreeNode<const N: usize, Data> = BookEntry<N, TreeNode<N, Data>>;

/// Référence immutable sur un noeud de l'arbre.
pub type RefTreeNode<const N: usize, Data> = BookReadEntry<N, TreeNode<N, Data>>;

/// Un noeud sur un arbre.
pub struct TreeNode<const N: usize, Data> {
    data: Data,
    pub parent: Option<WeakTreeNode<N, Data>>,
    pub children: Vec<WeakTreeNode<N, Data>>,
}

impl<const N: usize, Data> Deref for TreeNode<N, Data> {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<const N: usize, Data> DerefMut for TreeNode<N, Data> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

/// Un arbre, dont les noeuds sont stockés dans un livre.
/// 
/// Le paramètre N fixe la taille d'une page d'un livre.
pub struct Tree<const N: usize, Data>
where
    Data: 'static,
{
    /// Racine de l'arbre.
    maybe_root: Option<WeakTreeNode<N, Data>>,
    /// Noeuds stockés dans l'arbre.
    nodes: Book<N, TreeNode<N, Data>>,
}
impl<const N: usize, Data> Default for Tree<N, Data> {
    fn default() -> Self {
        Self {
            maybe_root: None,
            nodes: Book::default(),
        }
    }
}

impl<const N: usize, Data> Tree<N, Data> {
    /// Essaye d'emprunter la racine, si elle existe.
    /// 
    /// L'opération peut échouer si la racine est déjà empruntée en écriture.
    pub fn try_borrow_root(&self) -> Option<TreeResult<RefTreeNode<N, Data>>> {
        self.maybe_root.map(|root| root.try_read_upgrade().unwrap())
    }

    /// Retourne une référence faible vers la racine, si elle existe.
    pub fn root(&self) -> Option<WeakTreeNode<N, Data>> {
        self.maybe_root.clone()
    }   

    /// Ajoute un sous-arbre en partant du noeud définit à *from*.
    pub fn append_subtree(&mut self, mut from: MutTreeNode<N, Data>, other: Self) {
        self.nodes += other.nodes;
        
        if let Some(root) = other.root() {
            from.children.push(root);
        }
    }

    /// Crée un nouveau noeud.
    /// 
    /// Cette fonction ne crée pas de liens entre les noeuds (Parent/enfants).
    /// 
    /// Si aucune racine n'existe, le noeud prend sa place.
    /// 
    /// Retourne une référence faible vers le noeud.
    pub fn insert_node<D: Into<Data>>(&mut self, data: D) -> WeakTreeNode<N, Data> {
        let node = TreeNode {
            data: data.into(),
            parent: None,
            children: Vec::default(),
        };

        let node = MutTreeNode::weak_downgrade(&self.nodes.write(node));
        if self.maybe_root.is_none() {
            self.maybe_root = Some(node.clone());
        }

        node
    }
}
