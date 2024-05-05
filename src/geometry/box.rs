use super::{Edge, Rect};

#[derive(Default)]
pub struct Box<Unit> {
    pub content: Rect<Unit>,
    pub padding: Edge<Unit>,
    pub border: Edge<Unit>,
    pub margin: Edge<Unit>,
}
