pub mod edge;
pub mod r#box;

pub use r#box::BoxTree;
pub use edge::Edge;

use crate::dom;

pub enum Level {
    Block,
    Inline
}

/// Generates the layout of the document (Box tree)
pub fn layout(document: &dom::Document) -> BoxTree<'_, i64> { 
    let tree = BoxTree::<'_, i64>::default();

    tree
}

/// Generate a box from a document's node.
fn generate_box(document: &dom::Document, node: &dom::NodeId, tree: &mut BoxTree<'_, i64>) {

}