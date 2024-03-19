pub mod r#box;
pub mod edge;
pub mod formatting;
pub mod fragment;
pub mod text_sequence;

use std::{fmt::Display, ops::Deref};

pub use edge::Edge;
pub use fragment::{Fragment, FragmentTree};
pub use r#box::Box;
pub use text_sequence::TextSequence;

use crate::document::{self, Element};

use self::{
    formatting::{FormattingContextId, FormattingContexts},
    fragment::FragmentId,
};

/// Generates the formatting tree (fragment tree) of the document.
pub fn format(doc: &document::Document) -> FragmentTree<i64> {
    let mut fcs = formatting::FormattingContexts::default();
    let mut ftree = FragmentTree::<i64>::default();

    // Par défaut, on démarre dans un BFC pour le formattage.
    let root_fc = fcs.new_bfc();
    if let Some(root) = doc.root() {
        if let Some(root_fragment) = format_document_node(doc, root, &mut ftree, &root_fc, &mut fcs)
        {
            ftree.set_root(&root_fragment);
        }
    }

    ftree
}

/// Formate un noeud d'un document.
fn format_document_node(
    doc: &document::Document,
    doc_node_id: &document::NodeId,
    ftree: &mut FragmentTree<i64>,
    fc_id: &FormattingContextId,
    fcs: &mut FormattingContexts,
) -> Option<FragmentId> {
    match doc.borrow(doc_node_id).deref().deref() {
        document::DocumentNode::Text(text) => Some(ftree.new_text_sequence(text)),
        document::DocumentNode::Element(el) => format_document_element(doc, el, fc_id, fcs),
    }
}

/// Formate un élément d'un document.
fn format_document_element(
    doc: &document::Document,
    element: &Element,
    fc_id: &FormattingContextId,
    fcs: &mut FormattingContexts,
) -> Option<FragmentId> {
    // On ne génère pas de sous-arbre d'affichage si display: None.
    if element.style.display.is_none() {
        return None;
    }

    None
}
