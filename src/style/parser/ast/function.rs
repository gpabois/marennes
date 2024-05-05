use super::ComponentValue;

pub struct Function<'i> {
    pub name: &'i str,
    pub args: Vec<ComponentValue<'i>>,
}

impl Function<'_> {
    pub fn new(name: &'_ str) -> Self {
        Self {
            name,
            args: Vec::default(),
        }
    }
}
