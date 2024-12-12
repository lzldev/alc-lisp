use anyhow::anyhow;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub start: TokenPosition,
    pub end: TokenPosition,
}

impl Token {
    pub fn token_type(&self) -> &TokenType {
        return &self.token_type;
    }
}

#[derive(Clone, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub struct TokenPosition {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, Debug)]
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
