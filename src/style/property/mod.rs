mod font_family;
mod display;

use std::fmt::Display;

pub use font_family::*;
pub use display::*;

use crate::style::value::Keyword;

use super::Value;

const ALLOWED_KWS: &[Keyword] = &[
    Keyword::Initial,
    Keyword::Inherit,
    Keyword::Unset
];

pub enum ExplicitDefaulting {
    Initial, 
    Inherit,
    Unset
}

impl From<Keyword> for ExplicitDefaulting {
    fn from(value: Keyword) -> Self {
        match value {
            Keyword::Initial => ExplicitDefaulting::Initial,
            Keyword::Inherit => ExplicitDefaulting::Inherit,
            Keyword::Unset   => ExplicitDefaulting::Unset,
            _ => panic!("Wrong keyword for explicit defaulting")
        }
    }
}

#[derive(Default)]
pub struct Property<T> {
    pub initial:    T,
    pub specified:  Option<T>,
    pub computed:   Option<T>,
    pub used:       Option<T>,

    pub explicit_defaulting: Option<ExplicitDefaulting>
}

impl<T> std::fmt::Display for Property<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.initial)
    }
}

impl<T> Property<T> 
    where T: From<Value> + Default
{
    pub fn new<V>(arg: V) -> Self where Value: From<V>, V: Clone {
        let value = Value::from(arg);

        if let Some(&kw) = value.either::<Keyword>(ALLOWED_KWS) {
            Self {
                initial:    T::default(),
                specified:  None,
                computed:   None,
                used:       None,
                explicit_defaulting: Some(ExplicitDefaulting::from(kw))
            }
        }
        else {
            Self {
                initial: T::default(),
                specified: Some(T::from(value)),
                computed: None,
                used: None,
                explicit_defaulting: None
            }
        }
    }
}

