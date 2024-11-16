use std::collections::HashMap;

use anyhow::anyhow;

use crate::lexer::{self, Token, TokenPosition, TokenType};

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
        let mut nodes = Vec::<Node>::new();
        while !self.tokens.is_empty() {
            nodes.push(self.parse_expression()?);
        }

        // let root = if let Some(token) = self.tokens.last() {
        //     match token.token_type() {
        //         lexer::TokenType::Unknown => {
        //             return Err(anyhow!(
        //                 "found unknown token line:{}:{}", //TODO:Maybe make this return a invalid statement
        //                 token.start.line,
        //                 token.start.col,
        //             ));
        //         }
        //         _ => self.parse_expression()?,
        //     }
        // } else {
        //     return Err(anyhow!("not tokens to parse",));
        // };

        if self.tokens.len() > 0 {
            return Err(anyhow!(
                "not all tokens were consumed from the ast: still missing: {}",
                self.tokens.len()
            ));
        }

        Ok(Program {
            env: HashMap::new(),
            root: Node::Expression(nodes),
        })
    }

    fn parse_expression(&mut self) -> anyhow::Result<Node> {
        self.current_position.push(0);

        let token = self.tokens.pop().unwrap();

        let node = match token.token_type() {
            lexer::TokenType::LParen => {
                let mut nodes = Vec::<Node>::new();

                while self
                    .tokens
                    .last()
                    .is_some_and(|token| !matches!(token.token_type(), TokenType::RParen))
                {
                    nodes.push(self.parse_expression()?)
                }

                if let None = self.tokens.pop() {
                    let last_position = nodes
                        .last()
                        .and_then(|node| Some(node.last_char()))
                        .unwrap_or_else(|| &TokenPosition { line: 0, col: 0 });

                    return Err(anyhow!(
                        "unterminated expression at {}:{}",
                        last_position.line,
                        last_position.col
                    ));
                }

                return Ok(Node::Expression(nodes));
            }
            lexer::TokenType::RParen => return Err(anyhow!("trying to parse a RParen")),
            lexer::TokenType::LSquare => {
                let mut nodes = Vec::<Node>::new();

                while self
                    .tokens
                    .last()
                    .is_some_and(|token| !matches!(token.token_type(), TokenType::RSquare))
                {
                    nodes.push(self.parse_expression()?)
                }

                if let None = self.tokens.pop() {
                    let last_position = nodes
                        .last()
                        .and_then(|node| Some(node.last_char()))
                        .unwrap_or_else(|| &TokenPosition { line: 0, col: 0 });

                    return Err(anyhow!(
                        "unterminated list at {}:{}",
                        last_position.line,
                        last_position.col
                    ));
                }

                return Ok(Node::List(nodes));
            } //TODO:List
            lexer::TokenType::RSquare => return Err(anyhow!("trying to parse a RSquare")),
            // lexer::TokenType::SingleQuote => todo!(),
            lexer::TokenType::StringLiteral => Node::StringLiteral(token),
            lexer::TokenType::NumberLiteral => Node::NumberLiteral(token),
            lexer::TokenType::Word => match token.value.as_str() {
                "fn" => self.parse_function(Node::Word(token))?,
                _ => Node::Word(token),
            },
            lexer::TokenType::Unknown => Node::Invalid(token), //TODO:Error ?
            lexer::TokenType::Comment => return self.parse_expression(),
            _ => todo!(),
        };

        if let Node::Invalid(_) = node {
            self.errors.push(self.current_position.clone());
        }

        *(self.current_position.last_mut().unwrap()) += 1;

        if let None = self.current_position.pop() {
            return Err(anyhow!("popping too much of the current position"));
        };

        return Ok(node);
    }

    fn parse_function(&mut self, fn_word: Node) -> anyhow::Result<Node> {
        todo!()
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
    Expression(Vec<Node>), //TODO: Those lists should have a Token for the starting and ending
    List(Vec<Node>),
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

    fn last_char(&self) -> &TokenPosition {
        match self {
            Node::Invalid(token)
            | Node::StringLiteral(token)
            | Node::Word(token)
            | Node::NumberLiteral(token) => return &token.end,
            Node::Expression(vec) | Node::List(vec) => vec
                .last()
                .and_then(|node| Some(node.last_char()))
                .unwrap_or_else(|| &TokenPosition { line: 10, col: 0 }),
        }
    }
}
