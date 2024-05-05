mod at_rule;
mod block;
mod component_value;
mod function;
mod qualified_rule;
mod rule;

pub use at_rule::AtRule;
pub use block::Block;
pub use component_value::ComponentValue;
pub use function::Function;
pub use qualified_rule::QualifiedRule;
pub use rule::Rule;

pub use cssparser::Token;

#[derive(Default)]
pub struct Sheet<'i>(Vec<Rule<'i>>);

impl Sheet<'_> {
    pub fn add_rule(&mut self, rule: Rule<'_>) {
        self.0.push(rule);
    }
}
