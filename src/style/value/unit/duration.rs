use crate::style::StyleError;


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum DurationUnit {
    S,
    Ms
}

impl TryFrom<&str> for DurationUnit {
    type Error = StyleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "s" => Ok(Self::S),
            "ms" => Ok(Self::Ms),
            _ => Err(StyleError::InvalidValue(&["<duration-unit>"]))
        }
    }
}

impl std::fmt::Display for DurationUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DurationUnit::S => write!(f, "s"),
            DurationUnit::Ms => write!(f, "ms"),
        }
    }
}