use crate::style::{Style, StyleError};


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum LengthUnit {
    Absolute(AbsoluteLengthUnit),
    Relative(RelativeLengthUnit)
}

impl TryFrom<&str> for LengthUnit {
    type Error = StyleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(unit) = AbsoluteLengthUnit::try_from(value) {
            return Ok(Self::Absolute(unit))
        }

        if let Ok(unit) = RelativeLengthUnit::try_from(value) {
            return Ok(Self::Relative(unit))
        }

        Err(StyleError::InvalidValue(&["<length-unit>"]))
    }
}

impl std::fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthUnit::Absolute(unit) => write!(f, "{}", unit),
            LengthUnit::Relative(unit) => write!(f, "{}", unit),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum AbsoluteLengthUnit {
    Cm,
    Mm,
    Q,
    In,
    Pt,
    Pc,
    Px
}

impl TryFrom<&str> for AbsoluteLengthUnit {
    type Error = StyleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "cm" => Ok(Self::Cm),
            "mm" => Ok(Self::Mm),
            "Q" => Ok(Self::Q),
            "in" => Ok(Self::In),
            "Pt" => Ok(Self::Pt),
            "Pc" => Ok(Self::Pc),
            "Px" => Ok(Self::Px),
            _ => Err(StyleError::InvalidValue(&["<absolute-length-unit>"]))
        }
    }
}

impl std::fmt::Display for AbsoluteLengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbsoluteLengthUnit::Cm => write!(f, "cm"),
            AbsoluteLengthUnit::Mm => write!(f, "mm"),
            AbsoluteLengthUnit::Q => write!(f, "Q"),
            AbsoluteLengthUnit::In => write!(f, "in"),
            AbsoluteLengthUnit::Pt => write!(f, "pt"),
            AbsoluteLengthUnit::Pc => write!(f, "pc"),
            AbsoluteLengthUnit::Px => write!(f, "px"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RelativeLengthUnit {
    Viewport(ViewportRelativeLengthUnit),
    Font(FontRelativeLengthUnit)
}

impl TryFrom<&str> for RelativeLengthUnit {
    type Error = StyleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(unit) = ViewportRelativeLengthUnit::try_from(value) {
            return Ok(Self::Viewport(unit))
        }

        if let Ok(unit) = FontRelativeLengthUnit::try_from(value) {
            return Ok(Self::Font(unit))
        }

        Err(StyleError::InvalidValue(&["<relative-length-unit>"]))
    }
}

impl std::fmt::Display for RelativeLengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelativeLengthUnit::Viewport(unit) => write!(f, "{}", unit),
            RelativeLengthUnit::Font(unit) => write!(f, "{}", unit),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ViewportRelativeLengthUnit {
    Vw,
    Svw,
    Lvw,
    Dvw,
    Vh,
    Svh,
    Lvh,
    Dvh,
    Vi,
    Svi,
    Lvi,
    Dvi,
    Vb,
    Svb,
    Lvb,
    Dvb,
    Vmin,
    Svmin,
    Lvmin,
    Dvmin,
    Vmax,
    Svmax,
    Lvmax,
    Dvmax
}

impl TryFrom<&str> for ViewportRelativeLengthUnit {
    type Error = StyleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "vw" => Ok(Self::Vw),
            "svw" => Ok(Self::Svw),
            "lvw" => Ok(Self::Lvw),
            "dvw" => Ok(Self::Dvw),

            "vh" => Ok(Self::Vh),
            "svh" => Ok(Self::Svh),
            "lvh" => Ok(Self::Lvw),
            
            "vi" => Ok(Self::Vi),
            "svi" => Ok(Self::Svi),
            "dvi" => Ok(Self::Dvi),

            "vb" => Ok(Self::Vb),
            "svb" => Ok(Self::Svb),
            "lvb" => Ok(Self::Lvb),
            "dvb" => Ok(Self::Dvb),

            "vmin" => Ok(Self::Vmin),
            "svmin" => Ok(Self::Svmin),
            "lvmin" => Ok(Self::Lvmin),
            "dvmin" => Ok(Self::Dvmin),

            "vmax" => Ok(Self::Vmax),
            "svmax" => Ok(Self::Svmax),
            "lvmax" => Ok(Self::Lvmax),
            "dvmax" => Ok(Self::Dvmax),

            _ => Err(StyleError::InvalidValue(&["viewport-relative-length"]))
        }
    }
}

impl std::fmt::Display for ViewportRelativeLengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViewportRelativeLengthUnit::Vh => write!(f, "vh"),
            ViewportRelativeLengthUnit::Svh => write!(f, "svh"),
            ViewportRelativeLengthUnit::Lvh => write!(f, "lvh"),
            ViewportRelativeLengthUnit::Dvh => write!(f, "dvh"),
            ViewportRelativeLengthUnit::Vi => write!(f, "vi"),
            ViewportRelativeLengthUnit::Svi => write!(f, "svi"),
            ViewportRelativeLengthUnit::Lvi => write!(f, "lvi"),
            ViewportRelativeLengthUnit::Dvi => write!(f, "dvi"),
            ViewportRelativeLengthUnit::Vb => write!(f, "vb"),
            ViewportRelativeLengthUnit::Svb => write!(f, "svb"),
            ViewportRelativeLengthUnit::Lvb => write!(f, "lvb"),
            ViewportRelativeLengthUnit::Dvb => write!(f, "dvb"),
            ViewportRelativeLengthUnit::Vmin => write!(f, "vmin"),
            ViewportRelativeLengthUnit::Svmin => write!(f, "svmin"),
            ViewportRelativeLengthUnit::Lvmin => write!(f, "lvmin"),
            ViewportRelativeLengthUnit::Dvmin => write!(f, "dvmin"),
            ViewportRelativeLengthUnit::Vmax => write!(f, "vmax"),
            ViewportRelativeLengthUnit::Svmax => write!(f, "svmax"),
            ViewportRelativeLengthUnit::Lvmax => write!(f, "lvmax"),
            ViewportRelativeLengthUnit::Dvmax => write!(f, "dvmax"),
            ViewportRelativeLengthUnit::Vw => write!(f, "vw"),
            ViewportRelativeLengthUnit::Svw => write!(f, "svw"),
            ViewportRelativeLengthUnit::Lvw => write!(f, "lvw"),
            ViewportRelativeLengthUnit::Dvw => write!(f, "dvw"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum FontRelativeLengthUnit {
    Em,
    Rem,
    Ex,
    Rex, 
    Cap,
    Rcap,
    Ch,
    Rch,
    Ic,
    Ric, 
    Lh,
    Rlh
}

impl TryFrom<&str> for FontRelativeLengthUnit {
    type Error = StyleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "em" => Ok(Self::Em),
            "rem" => Ok(Self::Rem),
            "ex" => Ok(Self::Ex),
            "cap" => Ok(Self::Cap),
            "rcap" => Ok(Self::Rcap),
            "ch" => Ok(Self::Ch),
            "rch" => Ok(Self::Rch),
            "ic" => Ok(Self::Ic),
            "Ric" => Ok(Self::Ric),
            "Lh" => Ok(Self::Lh),
            "rlh" => Ok(Self::Rlh),
            _ => Err(StyleError::InvalidValue(&["<font-relative-length-unit>"]))
        }
    }
}

impl std::fmt::Display for FontRelativeLengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontRelativeLengthUnit::Em => write!(f, "em"),
            FontRelativeLengthUnit::Rem => write!(f, "rem"),
            FontRelativeLengthUnit::Ex => write!(f, "ex"),
            FontRelativeLengthUnit::Rex => write!(f, "rex"),
            FontRelativeLengthUnit::Cap => write!(f, "cap"),
            FontRelativeLengthUnit::Rcap => write!(f, "rcap"),
            FontRelativeLengthUnit::Ch => write!(f, "ch"),
            FontRelativeLengthUnit::Rch => write!(f, "rch"),
            FontRelativeLengthUnit::Ic => write!(f, "ic"),
            FontRelativeLengthUnit::Ric => write!(f, "ric"),
            FontRelativeLengthUnit::Lh => write!(f, "lh"),
            FontRelativeLengthUnit::Rlh => write!(f, "rlh"),
        }
    }
}