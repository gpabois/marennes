use std::ops::Deref;

/// Données pour la police par défaut.
static DEFAULT_FONT_REGULAR: &'static [u8] = include_bytes!("../../assets/fonts/LiberationSans/LiberationSans-Regular.ttf");

/// Un glyphe d'une police.
pub struct Glyph(rusttype::Glyph<'static>);

/// Une police chargée en mémoire.
#[derive(Clone)]
pub struct Font(rusttype::Font<'static>);

impl Default for Font {
    fn default() -> Self {
        Self(rusttype::Font::try_from_bytes(DEFAULT_FONT_REGULAR).unwrap())
    }
}

impl Deref for Font {
    type Target = rusttype::Font<'static>;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}