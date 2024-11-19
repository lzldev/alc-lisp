use std::{collections::HashMap, rc::Rc};

use anyhow::{anyhow, Context};
use objects::Object;

use crate::ast::Node;

pub mod builtins;
pub mod objects;

pub type Reference = Rc<Object>;
pub type Env = HashMap<String, Reference>;

pub struct Program {
    pub env: Env,
}

impl Program {
    pub fn new(env: Env) -> Self {
        return Self { env };
    }

    pub fn eval(&mut self, root: &Node) -> anyhow::Result<Object> {
        let mut last_result: Object = Object::Null;
        match root {
            Node::Expression(expressions) => {
                for exp in expressions.iter() {
                    last_result = self.parse_expression(exp)?;
                }
                Ok(last_result)
            }
            _ => Err(anyhow!("Root node is not a expression")),
        }
    }

    fn parse_expression(&mut self, node: &Node) -> anyhow::Result<Object> {
        match node {
            Node::Invalid(_) => return Ok(Object::Error("Evaluating Invalid Node".to_owned())),
            Node::Expression(vec) => {
                if vec.is_empty() {
                    return Ok(Object::Null);
                }

                let first = self.parse_expression(&vec[0])?;

                let len = vec.len();

                if len == 1 {
                    return Ok(first);
                }

                let mut args: Vec<Object> = Vec::with_capacity(len - 1);

                for exp in &vec[1..] {
                    args.push(self.parse_expression(exp)?);
                }

                match first {
                    Object::Builtin(f) => return Ok(f(args)),
                    Object::Function => {}
                    obj => return Err(anyhow!("Invalid type starting expression {:?}", obj)),
                }

                return Ok(first);
            }
            Node::List(vec) => {
                let mut items: Vec<Object> = Vec::with_capacity(vec.len());

                for item in vec.iter() {
                    items.push(self.parse_expression(item)?);
                }

                return Ok(Object::List(items));
            }
            Node::StringLiteral(token) => {
                let len = token.value.len();

                return Ok(Object::String(token.value[1..(len - 2)].to_owned()));
            }
            Node::NumberLiteral(token) => {
                let value = token
                    .value
                    .parse::<usize>()
                    .context("error parsing numberliteral:")?;

                return Ok(Object::Integer(value));
            }
            Node::FunctionLiteral {
                token,
                arguments,
                body,
            } => todo!(),
            Node::Word(token) => {
                let object = self.env.get(&token.value);

                return match object {
                    Some(v) => {
                        return Ok(v.as_ref().clone());
                    }
                    None => Ok(Object::Null),
                };
            }
        }
    }
}
