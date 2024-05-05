use crate::style::StyleError;

use super::{Gradient, Url, Value};

#[derive(Debug, PartialEq, Clone)]
pub enum Image {
    Url(Url),
    Gradient(Gradient),
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Image::Url(url) => write!(f, "{}", url),
            Image::Gradient(gradient) => write!(f, "{}", gradient),
        }
    }
}

impl TryFrom<Value> for Image {
    type Error = StyleError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Image(image) => Ok(image),
            Value::Url(url) => Ok(Self::from(url)),
            Value::Gradient(gradient) => Ok(Self::from(gradient)),
            _ => Err(StyleError::InvalidValue(&["image", "url", "gradient"])),
        }
    }
}

impl From<Image> for Value {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}

impl From<Url> for Image {
    fn from(value: Url) -> Self {
        Self::Url(value)
    }
}

impl From<Gradient> for Image {
    fn from(value: Gradient) -> Self {
        Self::Gradient(value)
    }
}
