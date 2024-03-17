pub mod r#box;
pub mod edge;

pub use edge::Edge;
pub use r#box::BoxTree;

use crate::dom;

pub enum Level {
    Block,
    Inline,
}

pub struct BlockFormattingContext;
pub struct InlineFormattingContext;

pub enum FormattingContext {
    Block(BlockFormattingContext),
    Inline(InlineFormattingContext),
}

/// Generates the formatting tree (box tree) of the document.
pub fn format(document: &dom::Document) -> BoxTree<i64> {
    let tree = BoxTree::<i64>::default();

    tree
}

/// Generate a box from a document's node.
fn generate_box(document: &dom::Document, node: &dom::NodeId, tree: &mut BoxTree<i64>) {}
