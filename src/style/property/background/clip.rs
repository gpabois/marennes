use crate::style::{Keyword, StyleError, Value};

const ALLOWED_KWS: &[Keyword] = &[
    Keyword::BorderBox,
    Keyword::PaddingBox,
    Keyword::ContentBox,
    Keyword::Text,
];

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum BackgroundClip {
    BorderBox,
    PaddingBox,
    ContentBox,
    Text,
}

impl TryFrom<Keyword> for BackgroundClip {
    type Error = StyleError;

    fn try_from(value: Keyword) -> Result<Self, Self::Error> {
        match value {
            Keyword::BorderBox => Ok(Self::BorderBox),
            Keyword::PaddingBox => Ok(Self::PaddingBox),
            Keyword::ContentBox => Ok(Self::ContentBox),
            Keyword::Text => Ok(Self::Text),
            _ => Err(StyleError::InvalidValue(&[
                "border-box",
                "padding-box",
                "content-box",
                "text",
            ])),
        }
    }
}

impl TryFrom<Value> for BackgroundClip {
    type Error = StyleError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let kw = value
            .iter_keywords()
            .find(Keyword::is_either_func(ALLOWED_KWS))
            .cloned()
            .ok_or(StyleError::InvalidValue(&[
                "border-box",
                "padding-box",
                "content-box",
                "text",
            ]))?;

        Self::try_from(kw)
    }
}

impl From<BackgroundClip> for Keyword {
    fn from(value: BackgroundClip) -> Self {
        match value {
            BackgroundClip::BorderBox => Self::BorderBox,
            BackgroundClip::PaddingBox => Self::PaddingBox,
            BackgroundClip::ContentBox => Self::ContentBox,
            BackgroundClip::Text => Self::Text,
        }
    }
}

impl From<BackgroundClip> for Value {
    fn from(value: BackgroundClip) -> Self {
        let kw: Keyword = Keyword::from(value);
        Self::from(kw)
    }
}

impl Default for BackgroundClip {
    fn default() -> Self {
        Self::BorderBox
    }
}
