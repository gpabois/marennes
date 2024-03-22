use crate::style::Style;

pub struct TextSequence {
    pub text: String,
    pub style: Style
}

impl TextSequence {
    pub fn new(text: &str, style: Style) -> Self {
        Self { text: text.into(), style }
    }
}
