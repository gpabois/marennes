#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Url(String);

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "url(\"{}\")", self.0)
    }
}
