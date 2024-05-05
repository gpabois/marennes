use super::{Block, Function, Token};

pub enum ComponentValue<'i> {
    Function(Function<'i>),
    Block(Block),
    Token(Token<'i>),
}

impl<'i> From<Token<'i>> for ComponentValue<'i> {
    fn from(value: Token<'i>) -> Self {
        Self::Token(value)
    }
}

impl<'i> From<Function<'i>> for ComponentValue<'i> {
    fn from(value: Function) -> Self {
        Self::Function(value)
    }
}

impl From<Block> for ComponentValue<'_> {
    fn from(value: Block) -> Self {
        Self::Block(value)
    }
}
