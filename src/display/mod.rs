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

use crate::{document::{self, Document, Element}, style::display::{InnerDisplayType, OuterDisplayType}};

use self::{
    formatting::{FormattingContextId, FormattingContexts},
    fragment::FragmentId,
};

/// Contexte global de formatage
pub struct GlobalContext<'a, Unit> where Unit: 'static {
    /// Document formaté
    doc: &'a Document,
    /// Collection des contextes de formatage.
    fcs: FormattingContexts,
    /// Arbre de fragments.
    ftree: FragmentTree<Unit>
}

/// Contexte local de formatage
pub struct LocalContext {
    /// Contexte de formatage localement applicable.
    fc_id: FormattingContextId,
    /// Boîte parente
    /// Boîte primordiale (boîte racine d'un élément)
}

/// Formate le document et génère un arbre de rendu.
pub fn format(doc: &document::Document) -> FragmentTree<i64> {
    let mut fcs = formatting::FormattingContexts::default();
    let mut ftree = FragmentTree::<i64>::default();

    let global = GlobalContext {doc, fcs, ftree};

    // Par défaut, on démarre dans un BFC pour le formatage.
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

/// Some elements aren’t rendered purely by CSS box concepts; 
/// for example, replaced elements (such as img), 
/// many form controls (such as input), and SVG elements.
/// [https://www.w3.org/TR/css-display-3/#unbox]
fn format_document_contents(    
    doc: &document::Document,
    element: &Element,
    fc_id: &FormattingContextId,
    fcs: &mut FormattingContexts,
) -> Option<FragmentId> {
    todo!("implémenter les éléments qui sont en dehors du modèle boite (display: contents)")
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

    // Un élément qui sort du modèle de boîte CSS (image, vidéo, etc.)
    if element.style.display.is_contents() {
       return format_document_contents(doc, element, fc_id, fcs)
    }

    // On s'occuper de l'affichage extérieur (outer display)
    // Cela gère la manière dont la boîte de l'élément 
    // se comporte dans le contexte de formatage actuel.
    // On peut unwrap car le outer type n'est pas disponible si None/Contents est défini (traité plus haut)
    let outer = element.style.display.outer_type().unwrap();
    match outer {
        // > The element generates a box that is block-level when placed in flow layout.
        OuterDisplayType::Block => todo!(),
        // > The element generates a box that is inline-level when placed in flow layout.
        OuterDisplayType::Inline => todo!(),
        // > The element generates an run-in box, which is a type of inline-level box 
        // > with special behavior that attempts to merge it into a subsequent block container. 
        // > See § 5 Run-In Layout for details. 
        OuterDisplayType::RunIn => todo!(),
    };

    // > If a <display-outside> value is specified but <display-inside> is omitted, 
    // > the element’s inner display type defaults to flow.
    // 
    // Source: [CSS Display Module Level 3](https://www.w3.org/TR/css-display-3/#outer-role).
    let inner = element.style.display.inner_type().unwrap_or(InnerDisplayType::Flow);
    match inner {
        // > The element lays out its contents using flow layout (block-and-inline layout).
        // > If its outer display type is inline or run-in, and it is participating in a block or inline formatting context, 
        // > then it generates an inline box.
        // >
        // > Otherwise it generates a block container box.
        // > Depending on the value of other properties (such as position, float, or overflow) 
        // > and whether it is itself participating in a block or inline formatting context, 
        // > it either establishes a new block formatting context for its contents or integrates 
        // > its contents into its parent formatting context. See CSS2.1 Chapter 9. [CSS2] A block container that establishes a new block formatting context is considered to have a used inner display type of flow-root.
        //
        // Source: [CSS Display Module Level 3](https://www.w3.org/TR/css-display-3/#outer-role)
        InnerDisplayType::Flow => {

        },
        InnerDisplayType::FlowRoot => todo!(),
        InnerDisplayType::Table => todo!(),
        InnerDisplayType::Flex => todo!(),
        InnerDisplayType::Grid => todo!(),
        InnerDisplayType::Ruby => todo!(),
    }
    None
}
