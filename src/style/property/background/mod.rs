mod attachment;
mod clip;
mod color;
mod image;

pub use attachment::*;
pub use clip::*;
pub use color::*;
pub use image::*;

use super::SpecProperty;

#[derive(Default)]
pub struct Background {
    pub attachment: BackgroundAttachment,
    pub clip: BackgroundClip,
    pub color: BackgroundColor,
}

#[derive(Default)]
pub struct SpecBackground {
    pub attachment: SpecProperty<BackgroundAttachment>,
    pub clip: SpecProperty<BackgroundClip>,
    pub color: SpecProperty<BackgroundColor>,
}
