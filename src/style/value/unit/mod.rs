mod length;
mod angle;
mod duration;
mod frequency;
mod resolution;

pub use length::*;
pub use angle::*;
pub use duration::*;
pub use frequency::*;
pub use resolution::*;

use crate::style::StyleError;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Unit {
    Length(LengthUnit),
    Angle(AngleUnit),
    Duration(DurationUnit),
    Frequency(FrequencyUnit),
    Resolution(ResolutionUnit)
}

impl From<LengthUnit> for Unit {
    fn from(value: LengthUnit) -> Self {
        Self::Length(value)
    }
}

impl TryFrom<&str> for Unit {
    type Error = StyleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(unit) = LengthUnit::try_from(value) {
            return Ok(Self::Length(unit));
        }

        if let Ok(unit) = AngleUnit::try_from(value) {
            return Ok(Self::Angle(unit))
        }

        if let Ok(unit) = DurationUnit::try_from(value) {
            return Ok(Self::Duration(unit))
        }

        if let Ok(unit) = FrequencyUnit::try_from(value) {
            return Ok(Self::Frequency(unit))
        }

        if let Ok(unit) = ResolutionUnit::try_from(value) {
            return Ok(Self::Resolution(unit))
        }

        Err(StyleError::InvalidValue(&["<unit>"]))
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Length(unit) => write!(f, "{}", unit),
            Unit::Angle(unit) => write!(f, "{}", unit),
            Unit::Duration(unit) => write!(f, "{}", unit),
            Unit::Frequency(unit) => write!(f, "{}", unit),
            Unit::Resolution(unit) => write!(f, "{}", unit),
        }
    }
}
