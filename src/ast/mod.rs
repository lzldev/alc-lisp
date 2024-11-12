use std::{collections::HashMap, fmt};

use anyhow::anyhow;

use crate::lexer::{self, Token};

#[derive(Clone, Debug)]
pub struct AST {
    tokens: Vec<Token>,
    current_position: ASTPosition,
    errors: Vec<ASTPosition>,
}

pub type ASTPosition = Vec<usize>;

impl AST {
    pub fn with_tokens(tokens: Vec<Token>) -> Self {
        let mut tokens = tokens;
        tokens.reverse(); //TODO:Fix this ?

        AST {
            tokens,
            current_position: vec![],
            errors: vec![],
        }
    }

    pub fn errors(&self) -> &Vec<ASTPosition> {
        &self.errors
    }

    pub fn has_errors(&self) -> bool {
        return !self.errors.is_empty();
    }

    pub fn print_errors(&self, root: &Node) {
        for (idx, position) in self.errors.iter().enumerate() {
            let node = root.node_at(position).unwrap();
            eprintln!("AST ERROR [{idx}]:{position:?}\n{node:#?}");
        }
    }

    pub fn parse(&mut self) -> anyhow::Result<Program> {
        let root = if let Some(token) = self.tokens.last() {
            match token.token_type() {
                lexer::TokenType::Unknown => {
                    return Err(anyhow!(
                        "found unknown token line:{}:{}", //TODO:Maybe make this return a invalid statement
                        token.start.line,
                        token.start.col,
                    ));
                }
                _ => self.parse_expression()?,
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
        self.current_position.push(0);
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

            if let Node::Invalid(_) = node {
                self.errors.push(self.current_position.clone());
            }

            nodes.push(node);
            *(self.current_position.last_mut().unwrap()) += 1;
        }

        if let None = self.current_position.pop() {
            return Err(anyhow!("popping too much of the current position"));
        };

        Ok(Node::Expression(nodes))
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    env: HashMap<String, Object>,
    pub root: Node,
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

impl Node {
    pub fn node_at(&self, position: &ASTPosition) -> anyhow::Result<&Node> {
        let mut node = self;

        let len = position.len();

        let mut i = 0;

        while i < len {
            node = match node {
                Node::Expression(vec) => vec
                    .get(position[i])
                    .ok_or_else(|| anyhow!("invalid index of node"))?,
                node_type => {
                    return Err(anyhow!(
                        "trying to get node position from node of type {:?}",
                        node_type
                    ))
                }
            };
            i += 1;
        }

        return Ok(node);
    }
}
