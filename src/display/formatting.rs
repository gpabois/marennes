pub struct BlockFormattingContext;
pub struct InlineFormattingContext;

pub enum FormattingContext {
    Block(BlockFormattingContext),
    Inline(InlineFormattingContext),
}

pub struct FormattingContexts {
    
}