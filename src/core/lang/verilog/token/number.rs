use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Decimal(String),
    Based(String),
    Real(String),
}

impl Number {
    pub fn is_negative(&self) -> bool {
        match self {
            Self::Based(s) => s
                .chars()
                .enumerate()
                .find(|(i, c)| i == &0 && c == &'-')
                .is_some(),
            Self::Decimal(s) => s
                .chars()
                .enumerate()
                .find(|(i, c)| i == &0 && c == &'-')
                .is_some(),
            Self::Real(s) => s
                .chars()
                .enumerate()
                .find(|(i, c)| i == &0 && c == &'-')
                .is_some(),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Decimal(s) => s.to_string(),
                Self::Based(b) => b.to_string(),
                Self::Real(r) => r.to_string(),
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum BaseSpec {
    Decimal(char),
    Hexadecimal(char),
    Octal(char),
    Binary(char),
}