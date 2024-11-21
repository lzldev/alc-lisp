use anyhow::anyhow;

use crate::lexer::{Token, TokenPosition};

use super::ASTPosition;

#[derive(Clone, Debug)]
pub enum Node {
    Word(Token),
    Invalid(Token),
    Expression(Vec<Node>), //TODO: Those lists should have a Token for the starting and ending
    List(Vec<Node>),
    StringLiteral(Token),
    NumberLiteral(Token),
    BooleanLiteral(Token),
    FunctionLiteral {
        token: Token,
        arguments: Vec<Node>,
        body: Box<Node>,
    },
}

impl Node {
    pub fn node_at(&self, position: &ASTPosition) -> anyhow::Result<&Node> {
        let mut node = self;

        let len = position.len();

        let mut i = 0;

        while i < len {
            node = match node {
                Node::Expression(vec) | Node::List(vec) => vec
                    .get(position[i])
                    .ok_or_else(|| anyhow!("invalid index of node {:?}", position))?,

                node_type => {
                    return Err(anyhow!(
                        "trying to get node position {:?} from node of type {:?}",
                        position,
                        node_type
                    ))
                }
            };
            i += 1;
        }

        return Ok(node);
    }

    pub fn last_char(&self) -> &TokenPosition {
        match self {
            Node::Invalid(token)
            | Node::StringLiteral(token)
            | Node::Word(token)
            | Node::NumberLiteral(token)
            | Node::BooleanLiteral(token) => return &token.end,
            Node::Expression(vec) | Node::List(vec) => vec
                .last()
                .and_then(|node| Some(node.last_char()))
                .unwrap_or_else(|| &TokenPosition { line: 10, col: 0 }),
            Node::FunctionLiteral { body, .. } => body.last_char(),
        }
    }
}
