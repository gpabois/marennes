pub struct TextSequence {
    pub text: String,
}

impl TextSequence {
    pub fn new(text: &str) -> Self {
        Self { text: text.into() }
    }
}
