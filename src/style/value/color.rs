use std::fmt::Display;

use crate::style::StyleError;

use super::Value;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RgbChannel {
    Percentage(f32),
    Integer(i8),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Hsl {
    hue: f32,
    saturation: f32,
    luminosity: f32,
    alpha: Option<f32>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rgb {
    pub red: RgbChannel,
    pub green: RgbChannel,
    pub blue: RgbChannel,
    pub alpha: Option<f32>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Hex(String),
    Rgb(Rgb),
    Hsl(Hsl),
    Transparent,
}

impl TryFrom<Value> for Color {
    type Error = StyleError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Color(color) => Ok(color),
            _ => Err(StyleError::InvalidValue(&[
                "rgba",
                "hsl",
                "hsla",
                "named-color",
                "hwb",
                "lch",
                "lab",
            ])),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Hex(hex) => write!(f, "#{}", hex),
            Color::Rgb(_) => todo!(),
            Color::Hsl(_) => todo!(),
            Color::Transparent => todo!(),
        }
    }
}
