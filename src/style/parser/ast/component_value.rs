use cssparser::{BasicParseError, CowRcStr, Parser, SourceLocation};

use crate::style::{consume, peek, CustomParseError, ParseResult, Token};

use super::{SimpleBlock, Function};

/// A component value
#[derive(Clone)]
pub enum ComponentValue<'i> {
    /// <function>
    Function(Function<'i>),
    /// {}, (), or [] blocks
    Block(SimpleBlock<'i>),
    /// Preserved token
    PreservedToken{
        location: SourceLocation,
        token: Token<'i>
    },
}


impl<'i> ComponentValue<'i> 
{
    pub fn location(&self) -> SourceLocation {
        match self {
            ComponentValue::Function(func) => func.location,
            ComponentValue::Block(bck) => bck.location,
            ComponentValue::PreservedToken { location, token: _ } => *location,
        }
    }

    pub fn try_into_token(self) -> ParseResult<'i, Token<'i>> {
        match self {
            ComponentValue::PreservedToken { location, token } => Ok(token),
            _ => Err(self.location().new_custom_error(CustomParseError::Expecting("<token>")))
        }      
    }

    pub fn try_into_ident(self) -> ParseResult<'i, CowRcStr<'i>>
    {
        match self {
            ComponentValue::PreservedToken { location, token: Token::Ident(id) } => Ok(id),
            _ => Err(self.location().new_custom_error(CustomParseError::Expecting("<ident>")))
        }
    }

    /// ','
    #[inline]
    pub fn is_comma(&self) -> bool {
        matches!(self, Self::PreservedToken { location: _, token: Token::Comma })
    }

    /// ':'
    #[inline]
    pub fn is_colon(&self) -> bool {
        matches!(self, Self::PreservedToken { location: _, token: Token::Colon })
    }

    #[inline]
    pub fn is_semicolon(&self) -> bool {
        matches!(self, Self::PreservedToken { location: _, token: Token::Semicolon })
    }

    fn consume_token(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
        let location = parser.current_source_location();
        let token = parser.next()?.clone();
        Ok(Self::PreservedToken { location, token })      
    }

    fn consume_simple_block(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
        consume(parser)?;
        Ok(Self::from(SimpleBlock::consume(parser)?))
    }

    fn consume_function(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
        consume(parser)?;
        Ok(Self::from(Function::consume(parser)?))
    }

    pub fn consume(parser: &mut Parser<'i, '_>) -> ParseResult<'i, Self> {
        match peek(parser)? {
            Token::Function(_) => Self::consume_function(parser),
            Token::ParenthesisBlock | Token::CurlyBracketBlock | Token::SquareBracketBlock => Self::consume_simple_block(parser),
            _ => Self::consume_token(parser)
        } 
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
