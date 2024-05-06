use crate::style::{ast::ComponentValue, Style, StyleError};

use cssparser::SourceLocation;
pub use cssparser::Token;

use super::{percentage, Dimension, Keyword, Number, Percentage, Unit, Value};

impl TryFrom<Token<'_>> for Value {
    type Error = StyleError;

    fn try_from(value: Token<'_>) -> Result<Self, Self::Error> {
        match value {
            Token::Ident(id) => {
                if let Ok(kw) = Keyword::try_from(id.as_ref()) {
                    // As a keyword
                    Ok(Value::from(kw))
                } else {
                    // As a string
                    Ok(Value::from(id.as_ref()))
                }
            },
            Token::Number { has_sign: _, value: _, int_value: _ } => {
                let nb = Number::try_from(value)?;
                Ok(Self::from(nb))
            },
            Token::Percentage { has_sign: _, unit_value: _, int_value:_ } => {
                let percentage = Percentage::try_from(value)?;
                Ok(Self::from(percentage))
            },
            Token::Dimension { has_sign: _, value: _, int_value: _, unit: _} => {
                let dim = Dimension::try_from(value)?;
                Ok(dim.into())
            },
            _ => Err(StyleError::InvalidValue(&["<value>"]))
        }
    }
}

impl TryFrom<Token<'_>> for Percentage {
    type Error = StyleError;

    fn try_from(value: Token<'_>) -> Result<Self, Self::Error> {
        match value {
            Token::Percentage { has_sign: _, unit_value, int_value: _ } => Percentage::try_from(unit_value),
            _ => Err(StyleError::InvalidValue(&["<dimension>"]))
        }
    }
}

impl TryFrom<Token<'_>> for Dimension {
    type Error = StyleError;

    fn try_from(value: Token<'_>) -> Result<Self, Self::Error> {
        let quantity = Number::try_from(&value)?;
        let unit = Unit::try_from(value)?;
        Ok(Dimension{quantity, unit})
    }
}

impl TryFrom<Token<'_>> for Unit {
    type Error = StyleError;

    fn try_from(value: Token<'_>) -> Result<Self, Self::Error> {
        match value {
            Token::Dimension { has_sign, value, int_value, unit } => Unit::try_from(unit.as_ref()),
            _ => Err(StyleError::InvalidValue(&["<dimension>"]))
        }
    }
}

impl TryFrom<Token<'_>> for Number {
    type Error = StyleError;

    fn try_from(value: Token<'_>) -> Result<Self, Self::Error> {
        if let Token::Number{has_sign, value, int_value} = value {
            return if let Some(int) = int_value {
                Ok(Self::from(int))
            } else {
                Ok(Self::from(value))
            }
        }

        Err(StyleError::InvalidValue(&["<number>"]))
    }
}

impl TryFrom<&Token<'_>> for Number {
    type Error = StyleError;

    fn try_from(value: &Token<'_>) -> Result<Self, Self::Error> {
        if let Token::Number{has_sign, value, int_value} = value {
            return if let Some(int) = int_value {
                Ok(Self::from(*int))
            } else {
                Ok(Self::from(*value))
            }
        }

        else if let Token::Dimension { has_sign, value, int_value, unit } = value {
            return if let Some(int) = int_value {
                Ok(Self::from(*int))
            } else {
                Ok(Self::from(*value))
            }        
        }

        Err(StyleError::InvalidValue(&["<number>"]))
    }
}