use std::collections::HashMap;

use anyhow::anyhow;

use crate::lexer::{self, Token};

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
        let root = if let Some(token) = self.tokens.last() {
            match token.token_type() {
                lexer::TokenType::LParen => self.parse_expression()?,
                lexer::TokenType::Unknown | _ => {
                    return Err(anyhow!(
                        "found unknown token line:{}:{}", //TODO:Maybe make this return a invalid statement
                        token.start.line,
                        token.start.col,
                    ));
                }
            }
        } else {
            return Err(anyhow!("not tokens to parse",));
        };

        if self.tokens.len() > 0 {
            return Err(anyhow!(
                "not all tokens were consumed from the ast: still missing: {}",
                self.tokens.len()
            ));
        }

        Ok(Program {
            env: HashMap::new(),
            root,
        })
    }

    fn parse_expression(&mut self) -> anyhow::Result<Node> {
        let mut nodes = Vec::<Node>::new();

        while let Some(token) = self.tokens.pop() {
            let node = match token.token_type() {
                lexer::TokenType::LParen => self.parse_expression()?,
                lexer::TokenType::RParen => break,
                lexer::TokenType::LSquare => todo!(), //TODO:List
                lexer::TokenType::RSquare => todo!(), // TOOD:Error
                lexer::TokenType::SingleQuote => todo!(),
                lexer::TokenType::StringLiteral => Node::StringLiteral(token),
                lexer::TokenType::NumberLiteral => Node::NumberLiteral(token),
                lexer::TokenType::Word => Node::Word(token),
                lexer::TokenType::Unknown => Node::Invalid(token), //TODO:Error ?
            };

            nodes.push(node);
        }

        if nodes.len() == 1 {
            return Ok(nodes.pop().unwrap());
        }

        Ok(Node::Expression(nodes))
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    env: HashMap<String, Object>,
    root: Node,
}

#[derive(Clone, Debug)]
pub enum Object {
    Number(usize),
    String(String),
}

#[derive(Clone, Debug)]
pub enum Node {
    Invalid(Token),
    Expression(Vec<Node>),
    StringLiteral(Token),
    NumberLiteral(Token),
    Word(Token),
}
