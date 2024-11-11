use std::time;

fn main() {
    let test_file = std::fs::read_to_string("./test.txt").expect("to open file");
    let mut lexer = Tokenizer::from_string(test_file);
    let start = time::Instant::now();
    lexer.parse();
    let took = time::Instant::now().duration_since(start);
    dbg!(lexer.tokens());
    dbg!(took);
}

#[derive(Clone, Debug)]
struct Tokenizer {
    internal: String,
    tokens: Vec<Token>,
}

#[derive(Clone, Debug)]
struct TokenPosition {
    line: usize,
    col: usize,
}

#[derive(Clone, Debug)]
struct Token {
    value: String,
    token_type: TokenType,
    start: TokenPosition,
    end: TokenPosition,
}

#[derive(Clone, Debug)]
enum TokenType {
    LParen,
    RParen,
    LSquare,
    RSquare,
    Plus,
    Minus,
    Slash,
    Asterisk,
    SingleQuote,
    StringLiteral,
    Word,
    NumericLiteral,
    Unknown,
}

impl TokenType {
    fn from_char(c: char) -> TokenType {
        match c {
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '[' => TokenType::LSquare,
            ']' => TokenType::RSquare,
            '\'' => TokenType::SingleQuote,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Asterisk,
            '/' => TokenType::Slash,
            _ => panic!("calling TokenType::from_char with unknown char"), //TODO: Return result instead
        }
    }
}

impl Tokenizer {
    pub fn from_string(string: String) -> Self {
        Tokenizer {
            internal: string,
            tokens: Vec::new(),
        }
    }

    fn parse(&mut self) {
        let mut iter = self
            .internal
            .chars()
            // .filter(|v| v != &'\n')
            .peekable();

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
                '+' | '-' | '/' | '*' | '(' | ')' | '[' | ']' | '\'' => self.tokens.push(Token {
                    value: value.to_string(),
                    token_type: TokenType::from_char(value),
                    start: TokenPosition {
                        line: line,
                        col: col - token_len,
                    },
                    end: TokenPosition {
                        line: line,
                        col: col,
                    },
                }),
                '"' => {
                    let mut string = value.to_string();

                    while iter.peek().is_some_and(|v| v != &'"') {
                        let letter = iter.next().unwrap();
                        col += 1;
                        string.push(letter);
                    }

                    iter.next().and_then(|f| {
                        col += 1;
                        string.push(f);
                        Option::<char>::None
                    }); // Push Last quote

                    self.tokens.push(Token {
                        value: string,
                        token_type: TokenType::StringLiteral,
                        start: TokenPosition {
                            line: line,
                            col: col_start,
                        },
                        end: TokenPosition {
                            line: line,
                            col: col,
                        },
                    })
                }
                c => {
                    if c.is_alphabetic() {
                        let mut word = c.to_string();
                        while iter
                            .peek()
                            .is_some_and(|v| v == &'_' || v == &'?' || v.is_alphanumeric())
                        {
                            let letter = iter.next().unwrap();
                            col += 1;
                            word.push(letter);
                        }

                        self.tokens.push(Token {
                            value: word,
                            token_type: TokenType::Word,
                            start: TokenPosition {
                                line: line,
                                col: col_start,
                            },
                            end: TokenPosition {
                                line: line,
                                col: col,
                            },
                        })
                    } else if c.is_numeric() {
                        let mut number = c.to_string();
                        while iter.peek().is_some_and(|v| v.is_alphanumeric()) {
                            let letter = iter.next().unwrap();
                            col += 1;
                            number.push(letter);
                        }

                        self.tokens.push(Token {
                            value: number,
                            token_type: TokenType::NumericLiteral,
                            start: TokenPosition {
                                line: line,
                                col: col_start,
                            },
                            end: TokenPosition {
                                line: line,
                                col: col,
                            },
                        })
                    } else {
                        self.tokens.push(Token {
                            value: value.to_string(),
                            token_type: TokenType::Unknown,
                            start: TokenPosition {
                                line: line,
                                col: col - token_len,
                            },
                            end: TokenPosition {
                                line: line,
                                col: col,
                            },
                        })
                    }
                }
            }
        }
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}
