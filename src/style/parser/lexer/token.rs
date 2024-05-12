use super::SourceLocation;

#[derive(Debug, PartialEq, Clone)]
pub struct Number {
    integer: String,
    decimal: String,
    exponent: String,
}

impl Number {
    pub fn new<S: ToString>(integer: S, decimal: S, exponent: S) -> Self {
        Self {
            integer: integer.to_string(),
            decimal: decimal.to_string(),
            exponent: exponent.to_string(),
        }
    }
}

impl From<&str> for Number {
    fn from(value: &str) -> Self {
        let mut parts = [String::default(), String::default(), String::default()];
        let mut sel = 0;

        for c in value.chars() {
            if c.is_numeric() {
                parts[sel].push(c);
            }

            if c == '.' {
                sel = 1;
            }

            if (c == 'e') || (c == 'E') {
                sel = 2;
            }

            if (c == '+' || c == '-') && parts[sel].is_empty() && (sel == 0 || sel == 2) {
                parts[sel].push(c);
            }
        }

        Self {
            integer: parts[0].clone(),
            decimal: parts[1].clone(),
            exponent: parts[2].clone(),
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.integer)?;
        if !self.decimal.is_empty() {
            write!(f, ".{}", self.decimal)?;
        }
        if !self.exponent.is_empty() {
            write!(f, "e{}", self.exponent)?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Dimension {
    pub number: Number,
    pub unit: String,
}

impl Dimension {
    pub fn new<N: Into<Number>, U: ToString>(number: N, unit: U) -> Self {
        Self {
            number: number.into(),
            unit: unit.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    String(String),
    BadString(String),
    AtKeyword(String),
    Ident(String),
    Delim(String),
    Number(Number),
    Dimension(Dimension),
    Percentage(Number),
    Function(String),
    Url(String),
    BadUrl(String),
    Whitespace,
    /// #<ident>
    Hash(String),
    /// (
    OpeningParenthesis,
    /// )
    ClosingParenthesis,
    /// {
    OpeningCurlyBracket,
    /// }
    ClosingCurlyBracket,
    /// [
    OpeningSquareBracket,
    /// ]
    ClosingSquareBracket,
    /// ,
    Comma,
    /// <--
    CDO,
    /// -->
    CDC,
    /// :
    Colon,
    /// ;
    Semicolon,
}

impl TokenKind {
    pub fn expect_ident(&self) -> &str {
        match self {
            Self::Ident(ident) => &ident,
            _ => panic!("not an ident token"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub location: SourceLocation,
    pub kind: TokenKind,
}

impl Token {
    #[inline]
    pub fn opening_parenthesis(location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::OpeningParenthesis,
        }
    }

    #[inline]
    pub fn closing_parenthesis(location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::ClosingParenthesis,
        }
    }

    #[inline]
    pub fn opening_curly_bracket(location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::OpeningCurlyBracket,
        }
    }

    #[inline]
    pub fn closing_curly_bracket(location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::ClosingCurlyBracket,
        }
    }

    #[inline]
    pub fn opening_square_bracket(location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::OpeningSquareBracket,
        }
    }

    #[inline]
    pub fn closing_square_bracket(location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::ClosingSquareBracket,
        }
    }

    #[inline]
    pub fn number<N: Into<Number>>(value: N, location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::Number(value.into()),
        }
    }

    #[inline]
    pub fn string<S: ToString>(value: S, location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::String(value.to_string()),
        }
    }

    #[inline]
    pub fn bad_string<S: ToString>(value: S, location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::BadString(value.to_string()),
        }
    }

    #[inline]
    pub fn hash<S: ToString>(value: S, location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::Hash(value.to_string()),
        }
    }

    #[inline]
    pub fn delim<S: ToString>(value: S, location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::Delim(value.to_string()),
        }
    }

    #[inline]
    pub fn dimension<N: Into<Number>, U: ToString>(
        number: N,
        unit: U,
        location: SourceLocation,
    ) -> Self {
        Self {
            location,
            kind: TokenKind::Dimension(Dimension::new(number, unit)),
        }
    }

    #[inline]
    pub fn percentage<N: Into<Number>>(number: N, location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::Percentage(number.into()),
        }
    }

    #[inline]
    pub fn comma(location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::Comma,
        }
    }

    #[inline]
    pub fn cdc(location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::CDC,
        }
    }

    #[inline]
    pub fn cdo(location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::CDO,
        }
    }

    #[inline]
    pub fn colon(location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::Colon,
        }
    }

    #[inline]
    pub fn ident<S: ToString>(value: S, location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::Ident(value.to_string()),
        }
    }

    #[inline]
    pub fn at_keyword<S: ToString>(value: S, location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::AtKeyword(value.to_string()),
        }
    }

    #[inline]
    pub fn semicolon(location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::Semicolon,
        }
    }

    #[inline]
    pub fn whitespace(location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::Whitespace,
        }
    }

    #[inline]
    pub fn function<S: ToString>(value: S, location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::Function(value.to_string()),
        }
    }

    #[inline]
    pub fn url<S: ToString>(value: S, location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::Url(value.to_string()),
        }
    }

    #[inline]
    pub fn bad_url<S: ToString>(value: S, location: SourceLocation) -> Self {
        Self {
            location,
            kind: TokenKind::BadUrl(value.to_string()),
        }
    }

    #[inline]
    pub fn is_delim_value<S: AsRef<str>>(&self, value: S) -> bool {
        match &self.kind {
            TokenKind::Delim(del) => del == value.as_ref(),
            _ => false,
        }
    }
}
