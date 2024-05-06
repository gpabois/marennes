use crate::style::StyleError;

use super::{Dimension, LengthUnit, Number, Percentage, Unit, Value};


#[derive(Debug, PartialEq, Clone)]
pub struct Length {
    pub quantity: Number,
    pub unit: LengthUnit
}

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.quantity, self.unit)
    }
}

impl From<Length> for Dimension {
    fn from(value: Length) -> Self {
        Self {
            quantity: value.quantity,
            unit: Unit::from(value.unit)
        }
    }
}

impl TryFrom<Value> for Length {
    type Error = StyleError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Dimension(dim) => dim.try_into(),
            Value::Length(length) => Ok(length),
            _ => Err(StyleError::InvalidValue(&["<length>"]))
        }
    }
}

impl TryFrom<Dimension> for Length {
    type Error = StyleError;

    fn try_from(value: Dimension) -> Result<Self, Self::Error> {
        if let Unit::Length(unit) = value.unit {
            return Ok(Self {
                quantity: value.quantity,
                unit
            });
        }

        Err(StyleError::InvalidValue(&["<length>"]))
    }
}

impl std::ops::Mul<Percentage> for Length {
    type Output = Self;

    fn mul(self, rhs: Percentage) -> Self::Output {
        Length {
            quantity: self.quantity * rhs,
            unit: self.unit
        }
    }
}