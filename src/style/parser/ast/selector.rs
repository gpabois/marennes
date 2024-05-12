use crate::style::{
    traits::{Lexer, Parser},
    TokenKind,
};

use super::ComponentValue;

///
///
/// # Grammar
/// <selector-list> = <complex-selector-list>
pub struct SelectorList();

///
///
/// # Grammar
/// <complex-selector> = <compound-selector> [ <combinator>? <compound-selector> ]*
///
/// <complex-selector> = <compound-selector>
///                    | <complex-selector> <cobminator> <compound-selector>
///                    | <complex-selector> <whitespace-token> <compound-selector>
pub enum ComplexSelector {
    Root(CompoundSelector),
    Combined(Box<ComplexSelector>, Option<String>, CompoundSelector),
}

///
///
/// # Grammar
/// <compound-selector> = [ <type-selector>? <subclass-selector>* [ <pseudo-element-selector> <pseudo-class-selector>* ]* ]!
pub struct CompoundSelector{
    type_selector: Option<TypeSelector>,
    subclass_selectors: Vec<SubclassSelector>,

};


pub struct TypeSelector;

pub struct SubclassSelector;

pub struct NsPrefix(String);

/// A pseudo-element selector
///
/// # Exemple
/// ::is("value")
pub struct PseudoElementSelector(PseudoClassSelector);

impl Parser<ComponentValue> for PseudoElementSelector {
    fn parse<L: Lexer<ComponentValue>>(lexer: &mut L) -> Self {
        if lexer
            .next()
            .and_then(|cv| cv.if_token(|tok| tok.is_delim_value(":")))
            .unwrap_or(false)
        {
            return Self(PseudoClassSelector::parse(lexer));
        }

        panic!("parsing error");
    }
}

/// A pseudo-class selector
///
/// # Example
/// :nth(0)
pub struct PseudoClassSelector(ComponentValue);

impl Parser<ComponentValue> for PseudoClassSelector {
    fn parse<L: Lexer<ComponentValue>>(lexer: &mut L) -> Self {
        if lexer
            .next()
            .and_then(|cv| cv.if_token(|tok| tok.is_delim_value(":")))
            .unwrap_or(false)
        {
            return Self(lexer.next().unwrap());
        }

        panic!("parsing error");
    }
}
