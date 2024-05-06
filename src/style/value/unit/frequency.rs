use crate::style::StyleError;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum FrequencyUnit {
    Hz,
    KHz
}

impl TryFrom<&str> for FrequencyUnit {
    type Error = StyleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Hz" => Ok(Self::Hz),
            "kHz" => Ok(Self::KHz),
            _ => Err(StyleError::InvalidValue(&["<frequency-unit>"]))
        }
    }
}

impl std::fmt::Display for FrequencyUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrequencyUnit::Hz => write!(f, "Hz"),
            FrequencyUnit::KHz => write!(f, "kHz"),
        }
    }
}