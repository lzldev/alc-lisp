use std::sync::Arc;

use anyhow::anyhow;

use crate::lexer::{Token, TokenPosition};

use super::ASTPosition;

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", content = "value")
)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
pub enum Node {
    Word(Token),
    Invalid(Token),
    Expression(Arc<[Node]>), //TODO: Those lists should have a Token for the starting and ending
    List(Arc<[Node]>),
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
    pub fn type_of(&self) -> &str {
        match self {
            Node::Word(_) => "word",
            Node::Invalid(_) => "invalid",
            Node::Expression(_) => "expression",
            Node::List(_) => "list",
            Node::StringLiteral(_) => "string",
            Node::NumberLiteral(_) => "number",
            Node::BooleanLiteral(_) => "boolean",
            Node::FunctionLiteral { .. } => "function",
        }
    }

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

        Ok(node)
    }

    pub fn first_char(&self) -> &TokenPosition {
        match self {
            Node::Invalid(token)
            | Node::StringLiteral(token)
            | Node::Word(token)
            | Node::NumberLiteral(token)
            | Node::BooleanLiteral(token) => &token.start,
            Node::Expression(vec) | Node::List(vec) => vec
                .first()
                .map(|node| node.first_char())
                .unwrap_or_else(|| &TokenPosition { line: 64, col: 64 }),
            Node::FunctionLiteral { body, .. } => body.first_char(),
        }
    }

    pub fn last_char(&self) -> &TokenPosition {
        match self {
            Node::Invalid(token)
            | Node::StringLiteral(token)
            | Node::Word(token)
            | Node::NumberLiteral(token)
            | Node::BooleanLiteral(token) => &token.end,
            Node::Expression(vec) | Node::List(vec) => vec
                .last()
                .map(|node| node.last_char())
                .unwrap_or_else(|| &TokenPosition { line: 10, col: 0 }),
            Node::FunctionLiteral { body, .. } => body.last_char(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Word(l0), Self::Word(r0)) => l0 == r0,
            (Self::Invalid(l0), Self::Invalid(r0)) => l0 == r0,
            (Self::Expression(l0), Self::Expression(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::StringLiteral(l0), Self::StringLiteral(r0)) => l0 == r0,
            (Self::NumberLiteral(l0), Self::NumberLiteral(r0)) => l0 == r0,
            (Self::BooleanLiteral(l0), Self::BooleanLiteral(r0)) => l0 == r0,
            (
                Self::FunctionLiteral {
                    token: l_token,
                    arguments: l_arguments,
                    body: l_body,
                },
                Self::FunctionLiteral {
                    token: r_token,
                    arguments: r_arguments,
                    body: r_body,
                },
            ) => l_token == r_token && l_arguments == r_arguments && l_body == r_body,
            _ => false,
        }
    }
}
