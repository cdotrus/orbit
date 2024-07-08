// — Number
// — String
// — Identifier

use super::super::super::lexer::Position;
use super::super::super::lexer::TrainCar;
use super::super::error::VerilogError;
use super::identifier::Identifier;
use super::keyword::Keyword;
use super::number::Number;
use super::operator::Operator;
use super::tokenizer::char_set;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum VerilogToken {
    Comment(Comment),
    Operator(Operator),
    Number(Number),
    String(String),
    Identifier(Identifier),
    Keyword(Keyword),
    StringLiteral(String),
    Directive(String),
    EOF,
}

impl Display for VerilogToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Comment(c) => c.to_string(),
                Self::Operator(o) => o.to_string(),
                Self::Number(n) => n.to_string(),
                Self::String(s) => s.to_string(),
                Self::Identifier(i) => i.to_string(),
                Self::Keyword(k) => k.to_string(),
                Self::StringLiteral(s) => s.to_string(),
                Self::Directive(d) => d.to_string(),
                Self::EOF => String::new(),
            }
        )
    }
}

impl VerilogToken {
    /// Checks if the element is a particular keyword `kw`.
    pub fn check_keyword(&self, kw: &Keyword) -> bool {
        match self {
            VerilogToken::Keyword(r) => r == kw,
            _ => false,
        }
    }

    pub fn is_eof(&self) -> bool {
        match self {
            VerilogToken::EOF => true,
            _ => false,
        }
    }

    pub fn is_directive(&self) -> bool {
        match self {
            VerilogToken::Directive(_) => true,
            _ => false,
        }
    }

    /// Accesses the underlying `Identifier`, if one exists.
    pub fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            VerilogToken::Identifier(id) => Some(id),
            _ => None,
        }
    }

    /// Accesses the underlying `Number`, if one exists.
    pub fn as_number(&self) -> Option<&Number> {
        match self {
            VerilogToken::Number(num) => Some(num),
            _ => None,
        }
    }

    /// Checks if the element is a particular delimiter `d`.
    pub fn check_delimiter(&self, d: &Operator) -> bool {
        match self {
            VerilogToken::Operator(r) => r == d,
            _ => false,
        }
    }

    pub fn as_comment(&self) -> Option<&Comment> {
        match self {
            VerilogToken::Comment(r) => Some(r),
            _ => None,
        }
    }

    pub fn is_comment(&self) -> bool {
        match self {
            VerilogToken::Comment(_) => true,
            _ => false,
        }
    }

    /// Takes the identifier from the token.
    pub fn take_identifier(self) -> Option<Identifier> {
        match self {
            Self::Identifier(i) => Some(i),
            _ => None,
        }
    }

    /// Takes the keyword from the token.
    pub fn take_keyword(self) -> Option<Keyword> {
        match self {
            Self::Keyword(kw) => Some(kw),
            _ => None,
        }
    }

    /// Casts into a keyword.
    pub fn as_keyword(&self) -> Option<&Keyword> {
        match self {
            Self::Keyword(kw) => Some(kw),
            _ => None,
        }
    }

    /// Checks if the current token type `self` is a delimiter.
    pub fn is_delimiter(&self) -> bool {
        match self {
            Self::Operator(_) => true,
            _ => false,
        }
    }

    /// Casts as a delimiter
    pub fn as_delimiter(&self) -> Option<&Operator> {
        match self {
            Self::Operator(d) => Some(d),
            _ => None,
        }
    }
}

impl VerilogToken {
    /// Walks through the stream to gather a `String` literal until finding the
    /// exiting character `br`.
    ///
    /// An escape is allowed by using \ before the `br`, i.e. "\"hello world\"".
    /// Assumes the first token to parse in the stream is not the `br` character.
    /// Allows for zero or more characters in result.
    fn consume_literal(
        train: &mut TrainCar<impl Iterator<Item = char>>,
        br: &char,
    ) -> Result<String, VerilogError> {
        let mut result = String::new();
        while let Some(c) = train.consume() {
            // detect escape sequence
            if &c == &char_set::ESC {
                result.push(c);
                if let Some(d) = train.consume() {
                    result.push(d);
                } else {
                    return Err(VerilogError::UnclosedLiteral(*br));
                }
            // exit the literal collection
            } else if &c == br {
                break;
            } else {
                result.push(c);
            }
        }
        Ok(result)
    }

    /// Attempts to match a string `s` to a valid delimiter.
    fn match_delimiter(s: &str) -> Result<Self, VerilogError> {
        match Operator::transform(s) {
            Some(d) => Ok(Self::Operator(d)),
            None => Err(VerilogError::InvalidSequence(s.to_string())),
        }
    }

    /// Captures the generic pattern production rule by passing a fn as `eval` to compare.
    ///
    /// This function allows for an empty result to be returned as `Ok`.
    /// - A ::= A { A }
    pub fn consume_value_pattern(
        train: &mut TrainCar<impl Iterator<Item = char>>,
        c0: Option<char>,
        eval: fn(&char) -> bool,
    ) -> Result<String, VerilogError> {
        let mut car = if let Some(c) = c0 {
            String::from(c)
        } else {
            String::new()
        };
        while let Some(c) = train.peek() {
            if eval(&c) == true {
                car.push(train.consume().unwrap());
            } else {
                break;
            }
        }
        Ok(car)
    }

    /// Captures an exponent.   
    ///
    /// Assumes the previous function has already consumed the previous character 'E' as `c0`.
    /// - exponent ::= E \[ + ] integer | E – integer  
    fn consume_exponent(
        train: &mut TrainCar<impl Iterator<Item = char>>,
        c0: char,
    ) -> Result<String, VerilogError> {
        // start with 'E'
        let mut expon = String::from(c0);
        // check for sign
        let sign = if let Some(c1) = train.consume() {
            if c1 != char_set::PLUS && c1 != char_set::MINUS && char_set::is_digit(&c1) == false {
                return Err(VerilogError::InvalidExponChar(c1));
            } else {
                c1
            }
        } else {
            return Err(VerilogError::EmptyExponChar);
        };
        // determine if c0 was a digit
        let c0 = if char_set::is_digit(&sign) == true {
            Some(sign)
        } else {
            // add the sign to the exponent
            expon.push(sign);
            None
        };
        let value = Self::consume_value_pattern(train, c0, char_set::is_digit_or_underscore)?;
        if value.is_empty() {
            Err(VerilogError::EmptyExponNumber)
        } else {
            expon.push_str(&value);
            Ok(expon)
        }
    }
}

impl VerilogToken {
    /// Captures Verilog Tokens: keywords and basic identifiers.
    ///
    /// Assumes the first `letter` char was the last char consumed before the function call.
    pub fn consume_word(
        train: &mut TrainCar<impl Iterator<Item = char>>,
        c0: char,
    ) -> Result<Self, VerilogError> {
        let word = Self::consume_value_pattern(train, Some(c0), char_set::is_identifier_character)?;
        if c0 == char_set::UNDER_SCORE {
            Ok(Self::Identifier(Identifier::Basic(word)))
        } else if c0 == char_set::DOLLAR_SIGN {
            Ok(Self::Identifier(Identifier::System(word)))
        } else {
            match Keyword::match_keyword(&word) {
                Some(kw) => Ok(Self::Keyword(kw)),
                None => Ok(Self::Identifier(Identifier::Basic(word))),
            }
        }
    }

    /// Captures a Verilog compiler directive, which may use a special keyword or identifier.
    /// Assumes the last consumed character was the grave accent character (`).
    ///
    /// A keyword/identifier must immediately follow from the grace accent character.
    pub fn consume_compiler_directive(
        train: &mut TrainCar<impl Iterator<Item = char>>,
    ) -> Result<Self, VerilogError> {
        let mut word = Self::consume_value_pattern(train, None, char_set::is_identifier_character)?;
        match word.as_ref() {
            // consume the remaining characters on the line (if exists)
            "define" | "include" | "timescale" | "ifdef" | "else" | "endif" => {
                while let Some(c) = train.consume() {
                    // cannot be vt, cr (\r), lf (\n)
                    if c == '\u{000B}' || c == '\u{000D}' || c == '\u{000A}' {
                        break;
                    } else {
                        word.push(c);
                    }
                }
                Ok(Self::Directive(word))
            }
            // a sequence of characters must follow the grave accent
            "" => Err(VerilogError::EmptyCompilerDirective),
            // assumed it was used to call macro identifier
            _ => Ok(Self::Directive(word)),
        }
    }

    /// Captures a Verilog escaped identifier introduced with a \ character.
    /// The \ character is assumed to be `c0`. Collects characters until it encounters whitespace.
    pub fn consume_escaped_identifier(
        train: &mut TrainCar<impl Iterator<Item = char>>,
    ) -> Result<Self, VerilogError> {
        let word = Self::consume_value_pattern(train, None, char_set::is_not_whitespace)?;
        Ok(Self::Identifier(Identifier::Escaped(word)))
    }

    /// Captures a string literal.
    ///
    /// Assumes the first double quote '\"' was the last char consumed before entering the function.
    pub fn consume_str_literal(
        train: &mut TrainCar<impl Iterator<Item = char>>,
    ) -> Result<Self, VerilogError> {
        let value = Self::consume_literal(train, &char_set::DOUBLE_QUOTE)?;
        Ok(VerilogToken::StringLiteral(value))
    }

    /// Collects a single-line comment (all characters after a `//` up until end-of-line).
    ///
    /// Assumes the opening '/' was the last char consumed before entering the function.
    /// Also assumes the next char is '/'.
    pub fn consume_oneline_comment(
        train: &mut TrainCar<impl Iterator<Item = char>>,
    ) -> Result<Self, VerilogError> {
        // skip over second '/'
        train.consume();
        // consume characters to form the comment
        let mut note = String::new();
        while let Some(c) = train.consume() {
            // cannot be vt, cr (\r), lf (\n)
            if c == '\u{000B}' || c == '\u{000D}' || c == '\u{000A}' {
                break;
            } else {
                note.push(c);
            }
        }
        Ok(Self::Comment(Comment::OneLine(note)))
    }

    /// Collects a block comment (all characters after a `/*` up until `*/`).
    ///
    /// Assumes the opening '/' char was the last char consumed before entering the function.
    /// Also assumes the next char is '*'.
    pub fn consume_block_comment(
        train: &mut TrainCar<impl Iterator<Item = char>>,
    ) -> Result<Self, VerilogError> {
        // skip over opening '*'
        train.consume().expect("Assumes '*' is the next character.");
        let mut note = String::new();
        while let Some(c) = train.consume() {
            // check if we are breaking from the comment
            if c == char_set::STAR {
                if let Some(c_next) = train.peek() {
                    // break from the comment
                    if c_next == &char_set::FWD_SLASH {
                        train.consume();
                        return Ok(Self::Comment(Comment::Block(note)));
                    }
                }
            }
            note.push(c);
        }
        Err(VerilogError::UnclosedBlockComment)
    }

    /// Walks through the possible interpretations for capturing a Verilog delimiter.
    ///
    /// If it successfully finds a valid Verilog delimiter, it will move the `loc` the number
    /// of characters it consumed.
    pub fn consume_operator(
        train: &mut TrainCar<impl Iterator<Item = char>>,
        c0: Option<char>,
    ) -> Result<Self, VerilogError> {
        // delimiter will have at most 3 characters
        let mut op_buf = String::with_capacity(3);
        if let Some(c) = c0 {
            op_buf.push(c);
        };
        // check the next character in the sequence
        while let Some(c) = train.peek() {
            match op_buf.len() {
                0 => match c {
                    // ambiguous characters...read another character (could be a 2 or 3 length operator)
                    '(' | ')' | '{' | '}' | '*' | '>' | '<' | '&' | '|' | '=' | '!' | '^' | '~' => {
                        op_buf.push(train.consume().unwrap())
                    }
                    // if it was an operator, take the character and increment the location
                    _ => return Self::match_delimiter(&String::from(train.consume().unwrap())),
                },
                1 => match op_buf.chars().nth(0).unwrap() {
                    '!' => match c {
                        // move on to next round (is a 2 or 3 length delimiter)
                        '=' => op_buf.push(train.consume().unwrap()),
                        // stop at 1
                        _ => return Ok(Self::match_delimiter(&op_buf)?),
                    },
                    '<' => match c {
                        // move on to next round (is a 2 or 3 length delimiter)
                        '<' | '=' => op_buf.push(train.consume().unwrap()),
                        // stop at 1
                        _ => return Ok(Self::match_delimiter(&op_buf)?),
                    },
                    '>' => match c {
                        // move on to next round (is a 2 or 3 length delimiter)
                        '>' | '=' => op_buf.push(train.consume().unwrap()),
                        // stop at 1
                        _ => return Ok(Self::match_delimiter(&op_buf)?),
                    },
                    '=' => match c {
                        // move on to next round (is a 2 or 3 length delimiter)
                        '=' => op_buf.push(train.consume().unwrap()),
                        // stop at 1
                        _ => return Ok(Self::match_delimiter(&op_buf)?),
                    },
                    _ => {
                        // try with 2
                        op_buf.push(*c);
                        if let Ok(op) = Self::match_delimiter(&op_buf) {
                            train.consume();
                            return Ok(op);
                        } else {
                            // revert back to 1
                            op_buf.pop();
                            return Self::match_delimiter(&op_buf);
                        }
                    }
                },
                2 => {
                    // try with 3
                    op_buf.push(*c);
                    if let Ok(op) = Self::match_delimiter(&op_buf) {
                        train.consume();
                        return Ok(op);
                    } else {
                        // revert back to 2 (guaranteed to exist)
                        op_buf.pop();
                        return Ok(Self::match_delimiter(&op_buf)?);
                    }
                }
                _ => panic!("Operator matching exceeds 3 characters"),
            }
        }
        // try when hiting end of stream
        Self::match_delimiter(&op_buf)
    }

    /// Captures Verilog tokens that are integer constants (decimal, based) or real constants (real).
    ///
    /// Assumes the incoming char `c0` was last char consumed.
    pub fn consume_number(
        train: &mut TrainCar<impl Iterator<Item = char>>,
        c0: char,
    ) -> Result<Self, VerilogError> {
        let mut number = String::new();

        if c0 != char_set::SINGLE_QUOTE {
            // get the first number
            number =
                Self::consume_value_pattern(train, Some(c0), char_set::is_digit_or_underscore)?;

            if let Some(c) = train.peek() {
                // real constant
                if c == &char_set::DOT || c == &char_set::BIG_E || c == &char_set::LIL_E {
                    // take the dot
                    if c == &char_set::DOT {
                        number.push(train.consume().unwrap());
                        // take more numbers as the fraction
                        let fraction = Self::consume_value_pattern(
                            train,
                            None,
                            char_set::is_digit_or_underscore,
                        )?;
                        if fraction.len() == 0 {
                            return Err(VerilogError::MissingNumbersAfterDecimalPoint);
                        }
                        number.push_str(&fraction);
                        // take the exponent (if exists)
                        if train
                            .peek()
                            .is_some_and(|d| d == &char_set::BIG_E || d == &char_set::LIL_E)
                        {
                            let e = train.consume().unwrap();
                            let expon = Self::consume_exponent(train, e)?;
                            number.push_str(&expon);
                            return Ok(Self::Number(Number::Real(number.trim().to_string())));
                        // no exponent (so we are done with the number)
                        } else {
                            return Ok(Self::Number(Number::Real(number.trim().to_string())));
                        }
                    // take the exponent (no fraction)
                    } else {
                        let e = train.consume().unwrap();
                        let expon = Self::consume_exponent(train, e)?;
                        number.push_str(&expon);
                        return Ok(Self::Number(Number::Real(number.trim().to_string())));
                    }
                } else {
                    let mut d = *c;
                    if char_set::is_letter(&d) == true {
                        return Err(VerilogError::InvalidCharInNumber(d));
                    }
                    // consume any whitespace
                    while char_set::is_whitespace(&d) == true {
                        number.push(train.consume().unwrap());
                        if let Some(f) = train.peek() {
                            d = *f;
                        } else {
                            // no more characters
                            return Ok(Self::Number(Number::Decimal(number.trim().to_string())));
                        }
                    }
                    // check the next character
                    if d != char_set::SINGLE_QUOTE {
                        return Ok(Self::Number(Number::Decimal(number.trim().to_string())));
                    } else {
                        number.push(train.consume().unwrap());
                    }
                }
            }
        } else {
            number.push(c0);
        }
        // handle based constant

        // the character to immediately come next must be a valid base specifier
        if let Some(c) = train.consume() {
            match c {
                's' | 'S' => {
                    number.push(c);
                    if let Some(c) = train.consume() {
                        match c {
                            'd' | 'D' | 'o' | 'O' | 'h' | 'H' | 'b' | 'B' => number.push(c),
                            _ => return Err(VerilogError::InvalidBaseSpecifier(c)),
                        }
                    } else {
                        return Err(VerilogError::MissingBaseSpecifier);
                    }
                }
                'd' | 'D' | 'o' | 'O' | 'h' | 'H' | 'b' | 'B' => number.push(c),
                _ => return Err(VerilogError::InvalidBaseSpecifier(c)),
            }
        } else {
            return Err(VerilogError::MissingBaseSpecifier);
        }

        if let Some(mut d) = train.peek() {
            // consume any whitespace
            while char_set::is_whitespace(&d) == true {
                number.push(train.consume().unwrap());
                if let Some(f) = train.peek() {
                    d = f;
                } else {
                    // no more characters
                    return Err(VerilogError::EmptyBaseConstNumber);
                }
            }
        } else {
            return Err(VerilogError::EmptyBaseConstNumber);
        }

        // take the remaining series of characters as the digits
        let value = Self::consume_value_pattern(
            train,
            None,
            char_set::is_digit_or_underscore_or_signal_char,
        )?;
        // make sure we have values
        match value.len() {
            0 => Err(VerilogError::EmptyBaseConstNumber),
            _ => {
                number.push_str(&value);
                Ok(Self::Number(Number::Based(number.trim().to_string())))
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Comment {
    OneLine(String),
    Block(String),
}

impl Comment {
    fn as_str(&self) -> &str {
        match self {
            Self::OneLine(note) => note.as_ref(),
            Self::Block(note) => note.as_ref(),
        }
    }

    /// Computes the ending position the cursor ends up in.
    pub fn ending_position(&self) -> Position {
        // begin with counting the opening delimiters (// or /*)
        let mut pos = Position::place(1, 2);
        let mut chars = self.as_str().chars();
        while let Some(c) = chars.next() {
            if char_set::is_newline(&c) == true {
                pos.next_line();
            } else {
                pos.next_col();
            }
        }
        match self {
            Self::OneLine(_) => (),
            // increment to handle the closing delimiters */
            Self::Block(_) => {
                pos.next_col();
                pos.next_col();
            }
        }
        pos
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OneLine(c) => write!(f, "//{}", c),
            Self::Block(c) => write!(f, "/*{}*/", c),
        }
    }
}