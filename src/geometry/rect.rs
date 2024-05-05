use super::Vec2D;

#[derive(Default)]
pub struct RectArea<Unit> {
    width: Unit,
    height: Unit,
}

#[derive(Default)]
pub struct Rect<Unit> {
    pub position: Vec2D<Unit>,
    pub area: RectArea<Unit>,
}
