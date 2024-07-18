use crate::core::lang::vhdl::token::ToColor;
use colored::ColoredString;
use colored::Colorize;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Delimiter {
    Ampersand,   // &
    SingleQuote, // '
    ParenL,      // (
    ParenR,      // )
    Star,        // *
    Plus,        // +
    Comma,       // ,
    Dash,        // -
    Dot,         // .
    FwdSlash,    // /
    Colon,       // :
    Terminator,  // ;
    Lt,          // <
    Eq,          // =
    Gt,          // >
    BackTick,    // `
    Pipe,        // | or ! VHDL-1993 LRM p180
    BrackL,      // [
    BrackR,      // ]
    Question,    // ?
    AtSymbol,    // @
    Arrow,       // =>
    DoubleStar,  // **
    VarAssign,   // :=
    Inequality,  // /=
    GTE,         // >=
    SigAssign,   // <=
    Box,         // <>
    SigAssoc,    // <=>
    CondConv,    // ??
    MatchEQ,     // ?=
    MatchNE,     // ?/=
    MatchLT,     // ?<
    MatchLTE,    // ?<=
    MatchGT,     // ?>
    MatchGTE,    // ?>=
    DoubleLT,    // <<
    DoubleGT,    // >>
}

impl Delimiter {
    /// Attempts to match the given string of characters `s` to a VHDL delimiter.
    pub fn transform(s: &str) -> Option<Self> {
        Some(match s {
            "&" => Self::Ampersand,
            "'" => Self::SingleQuote,
            "(" => Self::ParenL,
            ")" => Self::ParenR,
            "*" => Self::Star,
            "+" => Self::Plus,
            "," => Self::Comma,
            "-" => Self::Dash,
            "." => Self::Dot,
            "/" => Self::FwdSlash,
            ":" => Self::Colon,
            ";" => Self::Terminator,
            "<" => Self::Lt,
            "=" => Self::Eq,
            ">" => Self::Gt,
            "`" => Self::BackTick,
            "!" | "|" => Self::Pipe,
            "[" => Self::BrackL,
            "]" => Self::BrackR,
            "?" => Self::Question,
            "@" => Self::AtSymbol,
            "=>" => Self::Arrow,
            "**" => Self::DoubleStar,
            ":=" => Self::VarAssign,
            "/=" => Self::Inequality,
            ">=" => Self::GTE,
            "<=" => Self::SigAssign,
            "<>" => Self::Box,
            "<=>" => Self::SigAssoc,
            "??" => Self::CondConv,
            "?=" => Self::MatchEQ,
            "?/=" => Self::MatchNE,
            "?<" => Self::MatchLT,
            "?<=" => Self::MatchLTE,
            "?>" => Self::MatchGT,
            "?>=" => Self::MatchGTE,
            "<<" => Self::DoubleLT,
            ">>" => Self::DoubleGT,
            _ => return None,
        })
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Ampersand => "&",
            Self::SingleQuote => "'",
            Self::ParenL => "(",
            Self::ParenR => ")",
            Self::Star => "*",
            Self::Plus => "+",
            Self::Comma => ",",
            Self::Dash => "-",
            Self::Dot => ".",
            Self::FwdSlash => "/",
            Self::Colon => ":",
            Self::Terminator => ";",
            Self::Lt => "<",
            Self::Eq => "=",
            Self::Gt => ">",
            Self::BackTick => "`",
            Self::Pipe => "|",
            Self::BrackL => "[",
            Self::BrackR => "]",
            Self::Question => "?",
            Self::AtSymbol => "@",
            Self::Arrow => "=>",
            Self::DoubleStar => "**",
            Self::VarAssign => ":=",
            Self::Inequality => "/=",
            Self::GTE => ">=",
            Self::SigAssign => "<=",
            Self::Box => "<>",
            Self::SigAssoc => "<=>",
            Self::CondConv => "??",
            Self::MatchEQ => "?=",
            Self::MatchNE => "?/=",
            Self::MatchLT => "?<",
            Self::MatchLTE => "?<=",
            Self::MatchGT => "?>",
            Self::MatchGTE => "?>=",
            Self::DoubleLT => "<<",
            Self::DoubleGT => ">>",
        }
    }
}

impl Display for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ToColor for Delimiter {
    fn to_color(&self) -> ColoredString {
        self.to_string().normal()
    }
}