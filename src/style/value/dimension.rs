use super::{Number, Percentage, Unit, Value};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dimension {
    pub quantity: Number,
    pub unit: Unit,
}

impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.quantity, self.unit)
    }
}

impl std::ops::Mul<Percentage> for Dimension {
    type Output = Self;

    fn mul(self, rhs: Percentage) -> Self::Output {
        Dimension {
            quantity: self.quantity * rhs,
            unit: self.unit
        }
    }
}

impl From<Dimension> for Value {
    fn from(value: Dimension) -> Self {
        Self::Dimension(value)
    }
}