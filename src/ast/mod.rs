use std::collections::HashMap;

use anyhow::anyhow;

use crate::lexer::Token;

#[derive(Clone, Debug)]
pub struct AST {
    tokens: Vec<Token>,
    statements: Vec<Node>,
}

impl AST {
    pub fn with_tokens(tokens: Vec<Token>) -> Self {
        let mut tokens = tokens;
        tokens.reverse(); //TODO:Fix this ?

        AST {
            tokens,
            statements: Vec::new(),
        }
    }

    pub fn peek_token(&self) -> Option<&Token> {
        self.tokens.last()
    }

    pub fn parse(mut self) -> anyhow::Result<Program> {
        while let Some(token) = self.tokens.pop() {
            let node = match token.token_type() {
                crate::lexer::TokenType::LParen => self.parse_expression()?,
                crate::lexer::TokenType::Unknown => {
                    return Err(anyhow!(
                        "found unknown token line:{}:{}", //TODO:Maybe make this return a invalid statement
                        token.start.line,
                        token.start.col,
                    ));
                }
                token_type => {
                    return Err(anyhow!(
                        "found unknown token at: {} line:{}:{} {:?}",
                        self.tokens.len(),
                        token.start.line,
                        token.start.col,
                        token_type
                    ))
                } // crate::lexer::TokenType::RParen => todo!(),
                  // crate::lexer::TokenType::LSquare => todo!(),
                  // crate::lexer::TokenType::RSquare => todo!(),
                  // crate::lexer::TokenType::Plus => todo!(),
                  // crate::lexer::TokenType::Minus => todo!(),
                  // crate::lexer::TokenType::Slash => todo!(),
                  // crate::lexer::TokenType::Asterisk => todo!(),
                  // crate::lexer::TokenType::SingleQuote => todo!(),
                  // crate::lexer::TokenType::StringLiteral => todo!(),
                  // crate::lexer::TokenType::Word => todo!(),
                  // crate::lexer::TokenType::NumericLiteral => todo!(),
                  // crate::lexer::TokenType::Unknown => todo!(),
            };

            self.statements.push(node);
        }

        if self.tokens.len() > 0 {
            return Err(anyhow!(
                "not all tokens were consumed from the ast: still missing: {}",
                self.tokens.len()
            ));
        }

        Ok(Program {
            env: HashMap::new(),
            statements: self.statements,
        })
    }

    fn parse_expression(&mut self) -> anyhow::Result<Node> {
        let mut nodes = Vec::<Node>::new();

        while let Some(token) = self.tokens.pop() {
            let node = match token.token_type() {
                crate::lexer::TokenType::LParen => self.parse_expression()?,
                crate::lexer::TokenType::RParen => break,
                crate::lexer::TokenType::LSquare => todo!(), //TODO:List
                crate::lexer::TokenType::RSquare => todo!(), // TOOD:Error
                // crate::lexer::TokenType::Plus => todo!(), //TODO: Remove
                // crate::lexer::TokenType::Minus => todo!(), //TODO: Remove
                // crate::lexer::TokenType::Slash => todo!(), //TODO: Remove
                // crate::lexer::TokenType::Asterisk => todo!(), //TODO: Remove
                crate::lexer::TokenType::SingleQuote => todo!(),
                crate::lexer::TokenType::StringLiteral => Node::StringLiteral(token),
                crate::lexer::TokenType::NumericLiteral => Node::NumberLiteral(token),
                crate::lexer::TokenType::Word => Node::Word(token),
                crate::lexer::TokenType::Unknown => todo!(), //TODO:Error
                _ => todo!(),
            };

            nodes.push(node);
        }

        Ok(Node::Expression(nodes))
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    env: HashMap<String, Object>,
    statements: Vec<Node>,
}

#[derive(Clone, Debug)]
pub enum Object {
    Number(usize),
    String(String),
}

#[derive(Clone, Debug)]
pub enum Node {
    Expression(Vec<Node>),
    StringLiteral(Token),
    NumberLiteral(Token),
    Word(Token),
}
