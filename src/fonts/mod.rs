use std::ops::Deref;

/// Un glyphe d'une police.
pub struct Glyph(rusttype::Glyph<'static>);

/// Une police chargée en mémoire.
pub struct Font(rusttype::Font<'static>);

impl Deref for Font {
    type Target = rusttype::Font<'static>;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}