use crate::style::{Image, Keyword, StyleError, Value};

#[derive(Default, Clone, PartialEq)]
pub struct BackgroundImage(Option<Image>);

impl From<Image> for BackgroundImage {
    fn from(value: Image) -> Self {
        Self(Some(value))
    }
}

impl TryFrom<Keyword> for BackgroundImage {
    type Error = StyleError;

    fn try_from(value: Keyword) -> Result<Self, Self::Error> {
        if let Keyword::None = value {
            return Ok(Self(None));
        }

        Err(StyleError::InvalidValue(&["none"]))
    }
}

impl TryFrom<Value> for BackgroundImage {
    type Error = StyleError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let &Value::Keyword(Keyword::None) = &value {
            return Ok(Self(None));
        }

        if let Some(img) = value.into_iter().flat_map(Image::try_from).next() {
            return Ok(Self(Some(img)));
        }

        Err(StyleError::InvalidValue(&["<image>", "none"]))
    }
}

impl From<BackgroundImage> for Value {
    fn from(value: BackgroundImage) -> Self {
        match value.0 {
            Some(image) => Value::Image(image),
            None => Value::Keyword(Keyword::None),
        }
    }
}
