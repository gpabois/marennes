#[derive(Clone)]
pub enum Position {
    Relative,
    Absolute,
    Static,
    Fixed,
}

impl Default for Position {
    fn default() -> Self {
        Self::Static
    }
}
