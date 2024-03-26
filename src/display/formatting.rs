use std::ops::{Deref, DerefMut};

/// Block formatting context
#[derive(Default)]
pub struct BFC;

/// Inline formatting context
pub struct IFC;


pub enum FormattingContext {
    Block(BFC),
    Inline(IFC),
}

pub type FormattingContextId = BookItemId;

/// Une collection de contextes de formattage.
#[derive(Default)]
pub struct FormattingContexts(Book<50, FormattingContext>);

impl FormattingContexts {
    /// Crée un nouveau contexte de formattage de blocs.
    pub fn new_bfc(&mut self) -> FormattingContextId {
        self.write(FormattingContext::Block(BFC::default()))
    }
}

impl Deref for FormattingContexts {
    type Target = Book<50, FormattingContext>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FormattingContexts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
