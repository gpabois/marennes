use std::ops::Deref;

use crate::style::{Color, StyleError, Value};

pub struct BackgroundColor(Color);

impl Deref for BackgroundColor {
    type Target = Color;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for BackgroundColor {
    fn default() -> Self {
        Self(Color::Transparent)
    }
}

impl From<Color> for BackgroundColor {
    fn from(value: Color) -> Self {
        Self(value)
    }
}

impl TryFrom<Value> for BackgroundColor {
    type Error = StyleError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .into_iter()
                .flat_map(Color::try_from)
                .next()
                .ok_or(StyleError::InvalidValue(&["<color>"]))?,
        ))
    }
}

impl From<BackgroundColor> for Color {
    fn from(value: BackgroundColor) -> Self {
        value.0
    }
}

impl From<BackgroundColor> for Value {
    fn from(value: BackgroundColor) -> Self {
        Self::Color(value.into())
    }
}
