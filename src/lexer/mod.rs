use anyhow::anyhow;

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

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        let mut line = 0;

        for token in self.tokens.iter() {
            if token.start.line > line {
                line = token.start.line;
                out.push_str("\n");
            }
            out.push_str(" ");
            out.push_str(&token.value);
        }

        out
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
                    value: value.to_string(),
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
                        .and_then(|f| {
                            col += 1;
                            string.push(f);
                            Some(f)
                        })
                        .ok_or_else(|| {
                            anyhow!("unterminated string literal at {}:{}", line, col)
                        })?; // Push Last quote

                    self.tokens.push(Token {
                        value: string,
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
                        value: comment,
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
                            value: number,
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
                            value: word,
                            token_type: TokenType::Word,
                            start: TokenPosition {
                                line,
                                col: col_start,
                            },
                            end: TokenPosition { line, col },
                        })
                    } else {
                        self.tokens.push(Token {
                            value: value.to_string(),
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

        return Ok(());
    }
}

#[derive(Clone, Debug)]
pub struct TokenPosition {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub value: String,
    token_type: TokenType,
    pub start: TokenPosition,
    pub end: TokenPosition,
}

impl Token {
    pub fn token_type(&self) -> &TokenType {
        return &self.token_type;
    }
}

#[derive(Clone, Debug)]
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
