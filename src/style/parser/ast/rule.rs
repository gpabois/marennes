use super::{AtRule, QualifiedRule};

pub enum Rule<'i> {
    AtRule(AtRule<'i>),
    QualifiedRule(QualifiedRule),
}

impl<'i> From<AtRule<'i>> for Rule<'i> {
    fn from(value: AtRule<'i>) -> Self {
        Self::AtRule(value)
    }
}

impl From<QualifiedRule> for Rule<'_> {
    fn from(value: QualifiedRule) -> Self {
        Self::QualifiedRule(value)
    }
}
