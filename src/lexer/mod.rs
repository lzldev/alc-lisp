use std::fmt::Display;

use anyhow::anyhow;

mod token;
pub use token::*;

#[derive(Clone, Debug)]
pub struct Lexer {
    internal: String,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn from_string(string: String) -> Self {
        Lexer {
            internal: string,
            tokens: Vec::new(),
        }
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    fn is_word_symbol(c: char) -> bool {
        matches!(c, '+' | '-' | '/' | '*' | '_' | '=' | '?' | '!' | '<' | '>')
    }

    pub fn parse(&mut self) -> anyhow::Result<()> {
        let mut iter = self.internal.chars().peekable();

        let mut col = 0;
        let mut line = 1;

        while let Some(value) = iter.next() {
            let token_len = 1;
            col += token_len;
            let col_start = col;

            match value {
                ' ' => {}
                '\n' => {
                    col = 0;
                    line += 1
                }
                '(' | ')' | '[' | ']' | '\'' => self.tokens.push(Token {
                    value: value.to_string().into(),
                    token_type: TokenType::from_char(value)?,
                    start: TokenPosition {
                        line,
                        col: col - token_len,
                    },
                    end: TokenPosition { line, col },
                }),
                '"' => {
                    let mut string = value.to_string();

                    while iter.peek().is_some_and(|v| v != &'"') {
                        let letter = iter.next().unwrap();
                        col += 1;
                        string.push(letter);
                    }

                    iter.next()
                        .inspect(|&f| {
                            col += 1;
                            string.push(f);
                        })
                        .ok_or_else(|| {
                            anyhow!("unterminated string literal at {}:{}", line, col)
                        })?; // Push Last quote

                    self.tokens.push(Token {
                        value: string.into(),
                        token_type: TokenType::StringLiteral,
                        start: TokenPosition {
                            line,
                            col: col_start,
                        },
                        end: TokenPosition { line, col },
                    })
                }
                ';' => {
                    let mut comment = value.to_string();

                    while iter.peek().is_some_and(|v| v != &'\n') {
                        let letter = iter.next().unwrap();
                        col += 1;
                        comment.push(letter);
                    }

                    //TODO: Add ignore comments flag to the lexer; to skip adding it into the final result
                    self.tokens.push(Token {
                        value: comment.into(),
                        token_type: TokenType::Comment,
                        start: TokenPosition {
                            line,
                            col: col_start,
                        },
                        end: TokenPosition { line, col },
                    })
                }
                c => {
                    if c.is_numeric()
                        || ((c == '-' || c == '+') && iter.peek().is_some_and(|c| c.is_numeric()))
                    {
                        let mut number = c.to_string();
                        while iter.peek().is_some_and(|v| v.is_alphanumeric()) {
                            let letter = iter.next().unwrap();
                            col += 1;
                            number.push(letter);
                        }

                        self.tokens.push(Token {
                            value: number.into(),
                            token_type: TokenType::NumberLiteral,
                            start: TokenPosition {
                                line,
                                col: col_start,
                            },
                            end: TokenPosition { line, col },
                        })
                    } else if c.is_alphabetic() || Lexer::is_word_symbol(c) {
                        let mut word = c.to_string();

                        while iter
                            .peek()
                            .is_some_and(|v| Lexer::is_word_symbol(*v) || v.is_alphanumeric())
                        {
                            let letter = iter.next().unwrap();
                            col += 1;
                            word.push(letter);
                        }

                        self.tokens.push(Token {
                            value: word.into(),
                            token_type: TokenType::Word,
                            start: TokenPosition {
                                line,
                                col: col_start,
                            },
                            end: TokenPosition { line, col },
                        })
                    } else {
                        self.tokens.push(Token {
                            value: value.to_string().into(),
                            token_type: TokenType::Unknown,
                            start: TokenPosition {
                                line,
                                col: col - token_len,
                            },
                            end: TokenPosition { line, col },
                        })
                    }
                }
            }
        }

        Ok(())
    }
}

impl Display for Lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut line = 0;

        for token in self.tokens.iter() {
            if token.start.line > line {
                line = token.start.line;
                f.write_str("\n")?;
            }
            f.write_str(" ")?;
            f.write_str(&token.value)?;
        }

        Ok(())
    }
}
