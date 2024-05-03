use std::fmt::Display;


#[derive(Debug, Eq, PartialEq)]
pub enum Unit {
    Pixel,
    Point,
    Percentage,
    Em,
    Rem
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Pixel => write!(f, "px"),
            Unit::Point => write!(f, "pt"),
            Unit::Percentage => write!(f, "%"),
            Unit::Em => write!(f, "em"),
            Unit::Rem => write!(f, "rem"),
        }
    }
}

impl Unit {
    #[inline(always)]
    pub fn is_relative(&self) -> bool {
        matches!(self, Self::Percentage | Self::Em | Self::Rem)
    }
}

#[derive(Debug, PartialEq)]   
pub struct Length {
    pub quantity: f64,
    pub unit: Unit
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.quantity, self.unit)
    }
}

impl std::ops::Mul for Length {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let quantity = self.quantity * rhs.quantity;

        if self.unit == rhs.unit {
            return Self {
                unit: self.unit,
                quantity
            }
        }
        else if self.unit.is_relative() && !rhs.unit.is_relative() {
            return Self {
                unit: rhs.unit,
                quantity
            }
        }
        else if !self.unit.is_relative() && rhs.unit.is_relative() {
            return Self {
                unit: self.unit,
                quantity
            }
        }
        else {
            panic!("Lengths of different units are not multipliable.")
        }
    }
}