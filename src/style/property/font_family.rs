use std::fmt::Display;

use crate::style::{Keyword, Value};

const ALLOWED_KWS: &[Keyword] = &[
    Keyword::Serif,
    Keyword::SansSerif,
    Keyword::Monospace,
    Keyword::Cursive,
    Keyword::Fantasy,
    Keyword::SystemUi,
    Keyword::UiSerif,
    Keyword::UiSansSerif,
    Keyword::UiMonospace,
    Keyword::UiRounded,
    Keyword::Emoji,
    Keyword::Math,
    Keyword::Fangsong,
];

#[derive(Clone)]
pub struct FontFamily(Vec<Value>);

impl From<Value> for FontFamily {
    fn from(value: Value) -> FontFamily {
        let families: Vec<Value> = value
            .into_iter()
            .filter(|v| v.is_either(ALLOWED_KWS) | v.is_string())
            .collect();

        if families.is_empty() {
            return Self::default();
        }

        Self(families)
    }
}

impl From<FontFamily> for Value {
    fn from(value: FontFamily) -> Self {
        value.0.into_iter().collect()
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        Self::from(Value::from(Keyword::SystemUi))
    }
}

impl Display for FontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let families = self
            .0
            .iter()
            .cloned()
            .map(Value::from)
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "font-family: {}", families)
    }
}

