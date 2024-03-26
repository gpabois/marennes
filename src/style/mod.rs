pub mod display;
pub mod position;

pub use display::Display;
pub use position::Position;

use crate::fonts::Font;

#[derive(Clone, Default)]
/// Le style calculé pour un élément.
pub struct Style {
    pub position: Position,
    /// [https://www.w3.org/TR/css-display-3/]
    pub display: Display,
    /// La police chargée en mémoire
    pub font: Font
}
