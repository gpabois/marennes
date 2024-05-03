mod length;
mod keyword;
mod color;

pub use length::*;
pub use keyword::*;
pub use color::*;

#[derive(Debug, PartialEq)]   
pub enum Value {
    String(String),
    Length(Length),
    Keyword(Keyword),
    Color(Color),
    Array(Vec<Value>)
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(str) => write!(f, "\"{}\"", str),
            Value::Length(length) => write!(f, "{}", length),
            Value::Keyword(kw) => write!(f, "{}", kw),
            Value::Color(color) => write!(f, "{}", color),
            Value::Array(array) => {
                let str_array = array.iter().map(ToString::to_string).collect::<Vec<_>>().join(" ");
                write!(f, "{}", str_array)
            },
        }
    }
}

impl<V> FromIterator<V> for Value where Self: From<V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self::Array(iter.into_iter().map(Self::from).collect())
    }
}

impl From<Vec<Keyword>> for Value {
    fn from(value: Vec<Keyword>) -> Self {
        Value::Array(value.into_iter().map(Value::from).collect())
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

impl Value 
{
    pub fn either<'a, V>(&self, values: &'a [V]) -> Option<&'a V> where Self : From<V>, V: Clone {
        values.iter().find(|value| Self::from((*value).clone()) == *self)
    }

    pub fn is_either<V>(&self, values: &[V]) -> bool where Self : From<V>, V: Clone {
        values.iter().find(|value| Self::from((*value).clone()) == *self).is_some()
    }
    
    pub fn iter_keywords<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Keyword> + 'a> {
        match self {
            Self::Keyword(kw) => Box::new(std::iter::once(kw)),
            Self::Array(values) => {
                Box::new(values
                .iter()
                .map(|v| v.iter_keywords())
                .flatten())
            },
            _ => Box::new(std::iter::empty())
        }
    }

    pub fn iter_str<'a>(&'a self) -> Box<dyn Iterator<Item=&'a str> + 'a> {
        match self {
            Self::String(str) => Box::new(std::iter::once(str.as_str())),
            Self::Array(values) => {
                Box::new(values
                .iter()
                .map(|v| v.iter_str())
                .flatten())
            },
            _ => Box::new(std::iter::empty())
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