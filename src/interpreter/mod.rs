use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::{anyhow, Context, Result};
use objects::Object;

use crate::ast::Node;

mod constants;
pub use constants::*;

pub mod builtins;
pub mod objects;

pub type CallStack = Vec<RefCell<Env>>;
pub type Reference = Rc<Object>;
pub type Env = HashMap<String, Reference>;

macro_rules! map_rust_error {
    ($message:expr) => {
        |value: Reference| -> Result<Reference> {
            if is_error(&value) {
                return Err(anyhow!(concat!($message, ": {:?}"), value));
            } else {
                Ok(value)
            }
        }
    };
}

pub struct Program {
    pub env: Vec<RefCell<Env>>,
}

impl Program {
    pub fn new(global_env: Env) -> Self {
        let mut env = Vec::with_capacity(1024);

        env.push(RefCell::new(global_env));
        return Self { env };
    }

    pub fn eval(&mut self, root: &Node) -> anyhow::Result<Reference> {
        let mut last_result: Reference = NULL.clone();

        match root {
            Node::Expression(expressions) => {
                for exp in expressions.iter() {
                    last_result = self.parse_expression(exp)?;

                    if is_error(&last_result) {
                        return Err(anyhow!(
                            "error in expression at {}:{} : {:?}",
                            exp.last_char().line,
                            exp.last_char().line,
                            last_result
                        ));
                    }
                }
                Ok(last_result)
            }
            node => Ok(self.parse_expression(node)?),
        }
    }

    fn get_value(&mut self, name: &str) -> Reference {
        for env in self.env.iter_mut().rev() {
            let map = env.borrow_mut().get_mut();
            if let Some(value) = map.get(name) {
                return value.clone();
            }
        }

        return NULL.clone();
    }

    fn set_value(&mut self, name: String, value: Reference) {
        let env = self.env.last_mut().unwrap().borrow_mut().get_mut();
        env.insert(name, value);
    }

    fn parse_expression(&mut self, node: &Node) -> anyhow::Result<Reference> {
        match node {
            Node::Word(token) => Ok(self.get_value(token.value.as_str())),
            Node::BooleanLiteral(token) => match token.value.as_str() {
                "true" => Ok(TRUE.clone()),
                "false" => Ok(FALSE.clone()),
                _ => panic!("This should never happen"),
            },
            Node::StringLiteral(token) => {
                let len = token.value.len();

                return Ok(Reference::new(Object::String(
                    token.value[1..(len - 1)].to_owned(),
                )));
            }
            Node::NumberLiteral(token) => {
                return token
                    .value
                    .parse::<isize>()
                    .map(|v| Reference::new(Object::Integer(v)))
                    .context("error parsing numberliteral:");
            }
            Node::Invalid(_) => {
                return Ok(Reference::new(Object::Error(
                    "Evaluating Invalid Node".to_owned(),
                )))
            }
            Node::Expression(vec) => {
                if vec.is_empty() {
                    return Ok(NULL.clone());
                }

                let len = vec.len();

                if let Node::Word(word) = &vec[0] {
                    match word.value.as_str() {
                        "define" | "def" => {
                            if len == 1 || len != 3 {
                                return Ok(Reference::new(Object::Error(format!(
                                    "Invalid amount of arguments to define got:{} expected: 3",
                                    len
                                ))));
                            }

                            let name = match &vec[1] {
                                Node::Word(token) => token,
                                n => {
                                    return Ok(Reference::new(Object::Error(format!(
                                        "Invalid token for define: {:?} should be a word",
                                        n
                                    ))))
                                }
                            };

                            let value = self
                                .parse_expression(&vec[2])
                                .and_then(map_rust_error!("define value error"))?;

                            self.set_value(name.value.clone(), value);

                            return Ok(NULL.clone());
                        }
                        "if" => {
                            if len != 4 && len != 3 {
                                return Ok(Reference::new(Object::Error(format!(
                                    "Invalid amount of arguments to 'if' got: {}",
                                    len
                                ))));
                            }

                            let condition = self
                                .parse_expression(&vec[1])
                                .and_then(map_rust_error!("if condition error"))?;

                            let truthy = is_truthy(condition);

                            return if truthy {
                                self.parse_expression(&vec[2])
                            } else if len == 4 {
                                self.parse_expression(&vec[3])
                            } else {
                                Ok(NULL.clone())
                            }
                            .and_then(map_rust_error!("if result error"));
                        }
                        "do" => {
                            if len != 2 {
                                return Ok(Reference::new(Object::Error(format!(
                                    "Invalid amount of arguments to 'do' got: {}",
                                    len
                                ))));
                            }

                            return self.eval(&vec[1]);
                        }
                        _ => {}
                    }
                }

                let first = self
                    .parse_expression(&vec[0])
                    .and_then(map_rust_error!("in call to"))?;

                //TODO: only some expressions should do that
                if len == 1 {
                    return Ok(first);
                }

                let args = vec
                    .iter()
                    .skip(1)
                    .map(|exp| {
                        self.parse_expression(exp)
                            .and_then(map_rust_error!("function argument"))
                    })
                    .collect::<Result<Vec<_>>>()?;

                match first.as_ref() {
                    Object::Builtin { function } => return Ok(function(args)),
                    Object::Function {
                        env,
                        parameters,
                        body,
                    } => {
                        if args.len() != parameters.len() {
                            return Ok(Reference::new(Object::Error(format!("Invalid number of arguments passed into function got {} expected {}",args.len(),parameters.len()))));
                        }

                        self.env.push(env.clone());
                        for (idx, arg) in parameters.iter().enumerate() {
                            self.set_value(arg.clone(), args[idx].clone());
                        }
                        let ret = self.eval(&body)?;
                        self.env.pop();

                        return Ok(ret);
                    }
                    obj => return Err(anyhow!("Cannot call value of type {}", obj.type_of())),
                }
            }
            Node::List(vec) => {
                let items = vec
                    .iter()
                    .map(|item| {
                        self.parse_expression(item)
                            .and_then(map_rust_error!("list element"))
                    })
                    .collect::<Result<Vec<_>>>()?;

                return Ok(Reference::new(Object::List(items)));
            }
            Node::FunctionLiteral {
                token: _,
                arguments,
                body,
            } => {
                let arguments = arguments
                    .into_iter()
                    .map(|arg| {
                        let Node::Word(token) = arg else {
                            return Err(anyhow!("argument is not a word"));
                        };

                        return Ok(token.value.clone());
                    })
                    .collect::<Result<Vec<_>>>()?;

                let env = self.env.last().expect("to get last env").clone();

                return Ok(Reference::new(Object::Function {
                    env,
                    parameters: arguments,
                    body: (**body).clone(),
                }));
            }
        }
    }
}

fn bool_from_native(value: bool) -> Reference {
    if value {
        TRUE.clone()
    } else {
        FALSE.clone()
    }
}

#[inline(always)]
fn is_error(value: &Reference) -> bool {
    return matches!(value.as_ref(), Object::Error(_));
}

fn is_truthy(value: Reference) -> bool {
    match value.as_ref() {
        Object::Integer(v) => {
            if v != &0 {
                return true;
            }
        }
        Object::String(v) => {
            if !v.is_empty() {
                return true;
            }
        }
        Object::Bool(v) => {
            return *v;
        }
        Object::List(vec) => {
            if !vec.is_empty() {
                return true;
            }
        }
        _ => {}
    }

    return false;
}
