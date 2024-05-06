use super::{Percentage, Value};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Number {
    Int(i32),
    Float(f32)
}

impl std::ops::Mul<Percentage> for Number {
    type Output = Number;

    fn mul(self, rhs: Percentage) -> Self::Output {
        let percentage: f32 = rhs.into();
        self * percentage
    }
}

impl std::ops::Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Int(lhs), Number::Int(rhs)) => Self::Int(rhs * lhs),
            (Number::Int(lhs), Number::Float(rhs)) => Self::Int(((lhs as f32) * rhs) as i32),
            (Number::Float(lhs), Number::Int(rhs)) => Self::Int(((rhs as f32) * lhs) as i32),
            (Number::Float(lhs), Number::Float(rhs)) => Self::Float(rhs * lhs)
        }
    }
}

impl std::ops::Mul<f32> for Number {
    type Output = Number;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Number::Int(int) => {
                Self::Int(((int as f32) * rhs) as i32)
            },
            Number::Float(float) => Self::Float(float * rhs)
        }
    }
}

impl std::ops::Mul<i32> for Number {
    type Output = Number;

    fn mul(self, rhs: i32) -> Self::Output {
        match self {
            Number::Int(int) => {
                Self::Int(int * rhs)
            },
            Number::Float(float) => Self::Float(float * (rhs as f32))
        }
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Int(int) => write!(f, "{}", int),
            Number::Float(float) => write!(f, "{}", float),
        }
    }
}

impl From<Number> for Value {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::from(Number::from(value))
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::from(Number::from(value))
    }
}
