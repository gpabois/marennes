
/// An edge, a component of the box model.
pub struct Edge<Unit>{
    pub top:    Unit,
    pub bottom: Unit,
    pub right:  Unit,
    pub left:   Unit
}

impl<Unit> Default for Edge<Unit> where Unit: Default {
    fn default() -> Self {
        Self {
            top: Unit::default(),
            bottom: Unit::default(),
            right: Unit::default(),
            left: Unit::default()
        }
    }
}
