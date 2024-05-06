use std::convert::Infallible;

use cssparser::{CowRcStr, Parser, Token};

use crate::{iter::Splittable, style::{ParseResult, StyleError, Value}};

use super::{ComponentValue, SimpleBlock};

#[derive(Default)]
pub struct Declarations<'i>(Vec<Declaration<'i>>);

impl<'i> TryFrom<SimpleBlock<'i>> for Declarations<'i> {
    type Error = StyleError;

    fn try_from(value: SimpleBlock<'i>) -> Result<Self, Self::Error> {
        todo!()
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
}

impl<'i> FromIterator<ComponentValue<'i>> for Result<Declaration<'i>, Infallible> {
    fn from_iter<T: IntoIterator<Item = ComponentValue<'i>>>(iter: T) -> Self {
        let parts = iter
            .into_iter()
            .split_at(|cv| matches!(cv, ComponentValue::Token(Token::Colon)))
            .collect::<Vec<_>>();

        if let Token::Ident(name) = Token::try_from(parts.pop().expect("Missing name").next().unwrap())? {
            let decl = Declaration::new(name);
            let rhss = parts.pop().filter(|cv| matches!(cv, ComponentValue::Token(Token::Semicolon)));
            for rhs in  rhss{
                // We matched !important
                if let ComponentValue::Token(Token::Delim('!')) = rhs {
                    if let Some(ComponentValue::Token(Token::Ident("important"))) = rhss.pop() {
    
                    }
                }
            }

            return Ok(decl)
        }

        
    }
}
