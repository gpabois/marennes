mod background;
mod display;
mod font_family;

pub use background::*;
pub use display::*;
pub use font_family::*;

use crate::style::value::Keyword;

use super::Value;

const ALLOWED_KWS: &[Keyword] = &[Keyword::Initial, Keyword::Inherit, Keyword::Unset];

pub enum ExplicitDefaulting {
    Initial,
    Inherit,
    Unset,
}

impl From<Keyword> for ExplicitDefaulting {
    fn from(value: Keyword) -> Self {
        match value {
            Keyword::Initial => ExplicitDefaulting::Initial,
            Keyword::Inherit => ExplicitDefaulting::Inherit,
            Keyword::Unset => ExplicitDefaulting::Unset,
            _ => panic!("Wrong keyword for explicit defaulting"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum SpecProperty<T> {
    Initial,
    Inherit,
    Unset,
    Value(T),
}

impl<T> Default for SpecProperty<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::Value(T::default())
    }
}

impl<T> std::fmt::Display for SpecProperty<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecProperty::Initial => write!(f, "initial"),
            SpecProperty::Inherit => write!(f, "inherit"),
            SpecProperty::Unset => write!(f, "unset"),
            SpecProperty::Value(v) => v.fmt(f),
        }
    }
}

impl<T> From<SpecProperty<T>> for Value
where
    Value: From<T>,
{
    fn from(value: SpecProperty<T>) -> Self {
        match value {
            SpecProperty::Initial => Keyword::Initial.into(),
            SpecProperty::Inherit => Keyword::Inherit.into(),
            SpecProperty::Unset => Keyword::Unset.into(),
            SpecProperty::Value(value) => Value::from(value),
        }
    }
}

impl<T> SpecProperty<T>
where
    T: From<Value> + Default,
{
    pub fn new<V>(arg: V) -> Self
    where
        Value: From<V>,
        V: Clone,
    {
        let value = Value::from(arg);

        if let Some(&kw) = value.either::<Keyword>(ALLOWED_KWS) {
            match kw {
                Keyword::Inherit => Self::Inherit,
                Keyword::Initial => Self::Initial,
                Keyword::Unset => Self::Unset,
                _ => unreachable!(),
            }
        } else {
            Self::Value(T::from(value))
        }
    }
}
