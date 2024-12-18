use std::sync::Arc;

use anyhow::anyhow;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub struct Token {
    pub value: Arc<str>,
    pub token_type: TokenType,
    pub start: TokenPosition,
    pub end: TokenPosition,
}

impl Token {
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.token_type == other.token_type
            && self.start == other.start
            && self.end == other.end
    }
}

#[derive(Clone, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub struct TokenPosition {
    pub line: usize,
    pub col: usize,
}

impl PartialEq for TokenPosition {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.col == other.col
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub enum TokenType {
    LParen,
    RParen,
    LSquare,
    RSquare,
    SingleQuote,
    StringLiteral,
    Word,
    NumberLiteral,
    #[default]
    Unknown,
    Comment,
}

impl TokenType {
    pub fn from_char(c: char) -> anyhow::Result<TokenType> {
        match c {
            '(' => Ok(TokenType::LParen),
            ')' => Ok(TokenType::RParen),
            '[' => Ok(TokenType::LSquare),
            ']' => Ok(TokenType::RSquare),
            '\'' => Ok(TokenType::SingleQuote),
            _ => Err(anyhow!(
                "calling TokenType::from_char with unknown char ['{}']",
                c
            )), //TODO: Return result instead
        }
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
