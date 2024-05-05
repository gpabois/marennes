use crate::style::{Keyword, StyleError, Value};

const ALLOWED_KWS_ATTACHMENT: &[Keyword] = &[Keyword::Scroll, Keyword::Fixed, Keyword::Local];

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum BackgroundAttachment {
    Scroll,
    Fixed,
    Local,
}

impl TryFrom<Keyword> for BackgroundAttachment {
    type Error = StyleError;

    fn try_from(value: Keyword) -> Result<Self, Self::Error> {
        match value {
            Keyword::Scroll => Ok(BackgroundAttachment::Scroll),
            Keyword::Fixed => Ok(BackgroundAttachment::Fixed),
            Keyword::Local => Ok(BackgroundAttachment::Local),
            _ => Err(StyleError::InvalidValue(&["scroll", "fixed", "local"])),
        }
    }
}

impl TryFrom<Value> for BackgroundAttachment {
    type Error = StyleError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let kw = value
            .iter_keywords()
            .find(Keyword::is_either_func(ALLOWED_KWS_ATTACHMENT))
            .cloned()
            .ok_or(StyleError::InvalidValue(&["scroll", "fixed", "local"]))?;

        Self::try_from(kw)
    }
}

impl From<BackgroundAttachment> for Keyword {
    fn from(value: BackgroundAttachment) -> Self {
        match value {
            BackgroundAttachment::Scroll => Self::Scroll,
            BackgroundAttachment::Fixed => Self::Fixed,
            BackgroundAttachment::Local => Self::Local,
        }
    }
}

impl From<BackgroundAttachment> for Value {
    fn from(value: BackgroundAttachment) -> Self {
        let kw: Keyword = Keyword::from(value);
        Self::from(kw)
    }
}

impl Default for BackgroundAttachment {
    fn default() -> Self {
        Self::Scroll
    }
}
