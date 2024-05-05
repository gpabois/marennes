pub struct Vec2D<Unit> {
    pub x: Unit,
    pub y: Unit,
}

impl<Unit> Default for Vec2D<Unit>
where
    Unit: Default,
{
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
