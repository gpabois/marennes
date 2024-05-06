use crate::style::StyleError;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum AngleUnit {
    Deg,
    Grad,
    Rad,
    Turn
}

impl TryFrom<&str> for AngleUnit {
    type Error = StyleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "deg" => Ok(Self::Deg),
            "grad" => Ok(Self::Grad),
            "rad" => Ok(Self::Rad),
            "turn" => Ok(Self::Turn),
            _ => Err(StyleError::InvalidValue(&["<angle-unit>"]))
        }
    }
}

impl std::fmt::Display for AngleUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AngleUnit::Deg => write!(f, "deg"),
            AngleUnit::Grad => write!(f, "grad"),
            AngleUnit::Rad => write!(f, "rad"),
            AngleUnit::Turn => write!(f, "turn"),
        }
    }
}