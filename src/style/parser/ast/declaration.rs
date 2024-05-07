use std::{convert::Infallible, string::ParseError};

use cssparser::{CowRcStr, Parser, Token};

use crate::{iter::Splittable, style::{ParseResult, StyleError, Value}};

use super::{ComponentValue, SimpleBlock};

#[derive(Default)]
pub struct Declarations<'i>(Vec<Declaration<'i>>);

impl<'i> FromIterator<Declaration<'i>> for Declarations<'i> {
    fn from_iter<T: IntoIterator<Item = Declaration<'i>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'i> TryFrom<SimpleBlock<'i>> for Declarations<'i> {
    type Error = ParseError;

    fn try_from(value: SimpleBlock<'i>) -> Result<Self, Self::Error> {
        Ok(
            value
            .into_iter()
            .split_at(ComponentValue::is_semicolon)
            .flat_map(Declaration::consume)
            .collect()
        )
    }
}

impl<'i> Declarations<'i> {
    pub fn push(&mut self, decl: Declaration<'i>) {
        self.0.push(decl)
    }
}

pub struct Declaration<'i> {
    pub name: CowRcStr<'i>,
    pub values: Vec<ComponentValue<'i>>,
    pub important: bool
}

impl<'i> Declaration<'i> {
    pub fn new(name: CowRcStr<'i>) -> Self {
        Self {
            name,
            values: Default::default(),
            important: false
        }
    }   

    /// Parse an iterator of component values
    pub fn parse<I>(values: I) -> ParseResult<'i, Self> where I: Iterator<Item=ComponentValue<'i>> {
        
    }
}

