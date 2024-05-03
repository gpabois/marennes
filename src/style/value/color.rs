use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum RgbChannel {
    Percentage(f32),
    Integer(i8)
}

#[derive(Debug, PartialEq)]
pub struct Hsl {
    hue: f32,
    saturation: f32,
    luminosity: f32,
    alpha: Option<f32>
}

#[derive(Debug, PartialEq)]
pub struct Rgb {
    pub red:    RgbChannel,
    pub green:  RgbChannel,
    pub blue:   RgbChannel,
    pub alpha:  Option<f32>
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Hex(String),
    Rgb(Rgb),
    Hsl(Hsl)
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Hex(hex) => write!(f, "#{}", hex),
            Color::Rgb(_) => todo!(),
            Color::Hsl(_) => todo!(),
        }
    }
}