use cssparser::Parser;

use crate::style::{consume, peek, Dimension, Keyword, Number, ParseResult, StyleError, Token, Value};

use super::{SimpleBlock, Function};

/// A component value
pub enum ComponentValue<'i> {
    Function(Function<'i>),
    Block(SimpleBlock<'i>),
    Number(Number),
    Dimension(Dimension),
    Token(Token<'i>),
}

impl<'i> ComponentValue<'i> {
    pub fn consume(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
        match peek(parser)? {
            Token::Function(_) => Function::consume(parser).map(Self::from),
            Token::ParenthesisBlock | Token::CurlyBracketBlock | Token::SquareBracketBlock => {
                consume(parser)?;
                parser
                    .parse_nested_block(SimpleBlock::consume)
                    .map(Self::from)
            }
            tok => {
                parser.next()?;
                if let Ok(number) = Number::try_from(tok) {
                    Ok(Self::from(number))
                }
    
                else if let Ok(dim) = Dimension::try_from(tok) {
                    Ok(Self::from(dim))
                }
    
                Ok(Self::from(tok))
            }   
        } 
    }
}

impl TryFrom<ComponentValue<'_>> for Value {
    type Error = StyleError;

    fn try_from(value: ComponentValue<'_>) -> Result<Self, Self::Error> {
        match value {
            ComponentValue::Token(tok) => Self::try_from(tok),
            _ => Err(StyleError::InvalidValue(&["<value>"]))
        }
    }
}

impl<'i> TryFrom<ComponentValue<'i>> for Token<'i> {
    type Error = StyleError;

    fn try_from(value: ComponentValue<'i>) -> Result<Self, Self::Error> {
        match value {
            ComponentValue::Token(tok) => Ok(tok),
            _ => Err(StyleError::InvalidValue(&["<token>"]))
        }
    }
}

impl From<Number> for ComponentValue<'_> {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl From<Dimension> for ComponentValue<'_> {
    fn from(value: Dimension) -> Self {
        Self::Dimension(value)
    }
}

impl<'i> From<Function<'i>> for ComponentValue<'i> {
    fn from(value: Function<'i>) -> Self {
        Self::Function(value)
    }
}

impl<'i> From<SimpleBlock<'i>> for ComponentValue<'i> {
    fn from(value: SimpleBlock<'i>) -> Self {
        Self::Block(value)
    }
}
