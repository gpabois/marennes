mod error;
mod parser;
mod property;
mod value;

pub use error::*;
pub use property::*;
pub use value::*;

#[macro_export]
macro_rules! lookup {
    ($style:ident, "display") => {
        $style.display
    };
    ($style:ident, "background") => {
        $style.background
    };
    ($style:ident, "background-attachment") => {
        $style.background.attachment
    };
    ($style: ident, "background-clip") => {
        $style.background.clip
    };
    ($style:ident, "background-color") => {
        $style.background.color
    };
    ($style:ident, "background-image") => {
        $style.background.image
    };
    ($style:ident, "background-origin") => {
        $style.background.origin
    };
    ($style:ident, "background-position") => {
        $style.background.position
    };
    ($style:ident, "background-repeat") => {
        $style.background.repeat
    };
    ($style:ident, "background-size") => {
        $style.background.size
    };
}

#[derive(Default)]
pub struct Style {
    pub display: Display,
    pub background: Background,
}

#[cfg(test)]
mod tests {
    use crate::style::BackgroundAttachment;

    use super::Style;

    #[test]
    pub fn test_lookup() {
        let style = Style::default();

        assert_eq!(
            lookup!(style, "background-attachment"),
            BackgroundAttachment::default()
        );
    }
}
