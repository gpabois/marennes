mod at_rule;
mod block;
mod component_value;
mod function;
mod qualified_rule;
mod rule;
mod declaration;

pub use at_rule::AtRule;
pub use block::SimpleBlock;
pub use component_value::ComponentValue;
pub use function::Function;
pub use qualified_rule::QualifiedRule;
pub use rule::Rule;
pub use declaration::*;

pub use cssparser::Token;

#[derive(Default)]
pub struct Sheet<'i>(Vec<Rule<'i>>);

impl<'i> Sheet<'i> {
    pub fn add_rule(&mut self, rule: Rule<'i>) {
        self.0.push(rule);
    }
}
