mod color;
mod gradient;
mod image;
mod keyword;
mod length;
mod url;

pub use color::*;
pub use gradient::*;
pub use image::*;
pub use keyword::*;
pub use length::*;
pub use url::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Length(Length),
    Keyword(Keyword),
    Color(Color),
    Image(Image),
    Url(Url),
    Gradient(Gradient),
    Array(Vec<Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(str) => write!(f, "\"{}\"", str),
            Value::Length(length) => write!(f, "{}", length),
            Value::Keyword(kw) => write!(f, "{}", kw),
            Value::Color(color) => write!(f, "{}", color),
            Value::Image(image) => write!(f, "{}", image),
            Self::Array(array) => {
                let str_array = array
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "{}", str_array)
            }
            Value::Url(url) => write!(f, "{}", url),
            Value::Gradient(gradient) => write!(f, "{}", gradient),
        }
    }
}

impl IntoIterator for Value {
    type Item = Self;
    type IntoIter = Box<dyn Iterator<Item = Self>>;

    fn into_iter(self) -> Self::IntoIter {
        if let Self::Array(values) = self {
            return Box::new(values.into_iter());
        }

        Box::new(std::iter::once(self))
    }
}

impl<V> FromIterator<V> for Value
where
    Self: From<V>,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self::Array(iter.into_iter().map(Self::from).collect())
    }
}

impl From<Keyword> for Value {
    fn from(value: Keyword) -> Self {
        Self::Keyword(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl Value {
    pub fn either<'a, V>(&self, values: &'a [V]) -> Option<&'a V>
    where
        Self: From<V>,
        V: Clone,
    {
        values
            .iter()
            .find(|value| Self::from((*value).clone()) == *self)
    }

    pub fn is_either<V>(&self, values: &[V]) -> bool
    where
        Self: From<V>,
        V: Clone,
    {
        values
            .iter()
            .any(|value| Self::from((*value).clone()) == *self)
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self> + 'a> {
        if let Self::Array(values) = self {
            return Box::new(values.iter());
        }

        Box::new(std::iter::once(self))
    }

    pub fn iter_colors<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Color> + 'a> {
        match self {
            Self::Color(color) => Box::new(std::iter::once(color)),
            Self::Array(values) => Box::new(values.iter().flat_map(|v| v.iter_colors())),
            _ => Box::new(std::iter::empty()),
        }
    }

    pub fn iter_keywords<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Keyword> + 'a> {
        match self {
            Self::Keyword(kw) => Box::new(std::iter::once(kw)),
            Self::Array(values) => Box::new(values.iter().flat_map(|v| v.iter_keywords())),
            _ => Box::new(std::iter::empty()),
        }
    }

    pub fn iter_str<'a>(&'a self) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        match self {
            Self::String(str) => Box::new(std::iter::once(str.as_str())),
            Self::Array(values) => Box::new(values.iter().flat_map(|v| v.iter_str())),
            _ => Box::new(std::iter::empty()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Keyword, Value};

    #[test]
    pub fn test_001_iter_keywords() {
        let expected_kws = vec![Keyword::Block, Keyword::Inline];
        let value: Value = expected_kws.iter().cloned().collect();
        let kws: Vec<Keyword> = value.iter_keywords().cloned().collect();
        assert_eq!(kws, expected_kws);
    }
}
