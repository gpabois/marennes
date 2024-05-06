use crate::style::StyleError;


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ResolutionUnit {
    Dpi,
    Dpcm,
    Dppx
}

impl TryFrom<&str> for ResolutionUnit {
    type Error = StyleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "dpi" => Ok(Self::Dpi),
            "dpcm" => Ok(Self::Dpcm),
            "dppx" => Ok(Self::Dppx),
            _ => Err(StyleError::InvalidValue(&["<resolution-unit>"]))
        }
    }
}

impl std::fmt::Display for ResolutionUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolutionUnit::Dpi => write!(f, "dpi"),
            ResolutionUnit::Dpcm => write!(f, "dpcm"),
            ResolutionUnit::Dppx => write!(f, "dppx"),
        }
    }
}