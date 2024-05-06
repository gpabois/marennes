use crate::style::StyleError;

use super::Value;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage(f32);

impl Into<f32> for Percentage {
    fn into(self) -> f32 {
        self.0
    }
}

impl std::fmt::Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0 * 100.0)
    }
}

impl TryFrom<f32> for Percentage {
    type Error = StyleError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value <= 1.0 && value >= 0.0 {
            return Ok(Self(value))
        }

        Err(StyleError::InvalidValue(&["<percentage>"]))
    }
}

impl From<Percentage> for Value {
    fn from(value: Percentage) -> Self {
        Self::Percentage(value)
    }
}
