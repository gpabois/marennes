use std::fmt::Display;

use crate::style::Value;

pub struct FontFamily(Vec<String>);

impl From<Value> for FontFamily {
    fn from(value: Value) -> FontFamily {
        let families: Vec<String> = value.iter_str().map(|str| str.to_string()).collect();
        
        if families.len() == 0 {
            return Self::default()
        }

        return Self(families)
    }
}

impl Display for FontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let families = self.0.iter().cloned().map(Value::from).map(|v| v.to_string()).collect::<Vec<_>>().join(", ");
        write!(f, "font-family: {}", families)
    }
}

impl From<FontFamily> for Value {
    fn from(value: FontFamily) -> Self {
        value.0.into_iter().collect()
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        Self(vec!["system-ui".to_string()])
    }
}