use anyhow::{anyhow, Context};

use crate::lexer::{self, Token, TokenPosition, TokenType};

mod node;

pub use node::*;

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
            current_position: vec![0],
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

    pub fn parse(&mut self) -> anyhow::Result<Node> {
        let mut nodes = Vec::<Node>::new();
        while !self.tokens.is_empty() {
            nodes.push(self.parse_expression()?);
        }

        if self.tokens.len() > 0 {
            return Err(anyhow!(
                "not all tokens were consumed from the ast: still missing: {}",
                self.tokens.len()
            ));
        }

        return Ok(Node::Expression(nodes));
    }

    fn parse_expression(&mut self) -> anyhow::Result<Node> {
        let token = self
            .tokens
            .pop()
            .ok_or_else(|| return anyhow!("no expression"))?;

        if let TokenType::Comment = token.token_type() {
            return self.parse_expression();
        }

        let node = match token.token_type() {
            lexer::TokenType::LParen => {
                let mut nodes = Vec::<Node>::new();

                self.current_position.push(0);

                while self
                    .tokens
                    .last()
                    .is_some_and(|token| !matches!(token.token_type(), TokenType::RParen))
                {
                    nodes.push(self.parse_expression()?);
                }

                self.current_position.pop();

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

                Node::Expression(nodes)
            }
            lexer::TokenType::RParen => {
                return Err(anyhow!(
                    "trying to parse a RParen {}:{}",
                    token.end.line,
                    token.end.col
                ))
            }
            lexer::TokenType::LSquare => {
                let mut nodes = Vec::<Node>::new();

                self.current_position.push(0);

                while self
                    .tokens
                    .last()
                    .is_some_and(|token| !matches!(token.token_type(), TokenType::RSquare))
                {
                    nodes.push(self.parse_expression()?)
                }

                self.current_position.pop();

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

                Node::List(nodes)
            }
            lexer::TokenType::RSquare => {
                return Err(anyhow!(
                    "trying to parse a RSquare {}:{}",
                    token.end.line,
                    token.end.col
                ))
            }
            lexer::TokenType::StringLiteral => Node::StringLiteral(token),
            lexer::TokenType::NumberLiteral => Node::NumberLiteral(token),
            lexer::TokenType::Word => match token.value.as_str() {
                "fn" => self.parse_function(token)?,
                "true" | "false" => Node::BooleanLiteral(token),
                _ => Node::Word(token),
            },
            lexer::TokenType::Unknown => Node::Invalid(token), //TODO:Error ?
            lexer::TokenType::Comment => self.parse_expression()?, // Skip
            _ => todo!(),
        };

        if let Node::Invalid(_) = node {
            self.errors.push(self.current_position.clone());
        }

        *(self.current_position.last_mut().unwrap()) += 1;

        return Ok(node);
    }

    fn parse_function(&mut self, fn_word: Token) -> anyhow::Result<Node> {
        let arguments = self
            .parse_expression()
            .context("invalid function arguments:")?;

        let Node::List(words) = arguments else {
            return Err(anyhow!("invalid function declaration: invalid arguments"));
        };

        if words.iter().any(|node| !matches!(node, Node::Word(_))) {
            return Err(anyhow!(
                "invalid function arguments: arguments should only be identifiers"
            ));
        }

        let mut body = self.parse_expression().context("invalid function body:")?;

        if !matches!(body, Node::Expression(_)) {
            body = Node::Expression(vec![body])
        }

        return Ok(Node::FunctionLiteral {
            token: fn_word,
            arguments: words,
            body: Box::new(body),
        });
    }
}
