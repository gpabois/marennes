pub mod display;
pub mod position;

pub use display::Display;
pub use position::Position;

#[derive(Clone, Default)]
/// Computed style
pub struct Style {
    pub position: Position,
    /// [https://www.w3.org/TR/css-display-3/]
    pub display: Display,
}
