use crate::{fonts::{Font, Glyph}, style::Style};

/// Une séquence de texte.
pub struct TextSequence {
    pub text: String,
    pub style: Style,
    /// Référence vers la police chargée en mémoire qui coche les critères.
    pub font: Font,
    /// Le tableau des glyphes
    pub glyphes: Vec<Glyph>
}

impl TextSequence {
    pub fn new(text: &str, style: Style) -> Self {
        Self { 
            text: text.into(), 
            style 
        }
    }
}
