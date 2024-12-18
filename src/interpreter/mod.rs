use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use anyhow::{anyhow, Context, Result};
use objects::Object;
use once_cell::sync::Lazy;

use crate::ast::Node;

mod constants;
pub use constants::*;

pub mod builtins;
pub mod objects;

// pub type CallStack = Vec<RefCell<Env>>;
pub type Reference = Arc<Object>;
pub type EnvReference = Arc<EnvReferenceInner>;
pub type EnvReferenceInner = RwLock<Env>;
pub type Env = HashMap<String, Reference>;

static NUMBER_LOOKUP_TABLE: Lazy<Mutex<HashMap<String, Reference>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

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
    env: CallStack,
}

const STACK_SIZE: usize = 1024;

#[derive(Debug, Clone)]
pub struct CallStack {
    stack: [EnvReference; STACK_SIZE],
    sp: usize,
}

impl CallStack {
    pub fn new(initial: EnvReference) -> Self {
        let mut stack =
            std::array::from_fn(|_| EnvReference::new(EnvReferenceInner::new(Env::new())));
        stack[0] = initial;

        Self { stack, sp: 0 }
    }

    pub fn push_env(&mut self, env: EnvReference) {
        self.sp += 1;
        self.stack[self.sp] = env;
    }
    pub fn pop_env(&mut self) {
        self.sp -= 1;
    }
    pub fn current_env(&self) -> &EnvReference {
        &self.stack[self.sp]
    }
    pub fn current_env_mut(&mut self) -> &mut EnvReference {
        &mut self.stack[self.sp]
    }

    pub fn active_slice(&self) -> &[EnvReference] {
        return &self.stack[0..=self.sp];
    }
    pub fn active_slice_mut(&mut self) -> &mut [EnvReference] {
        return &mut self.stack[0..=self.sp];
    }
}

impl Program {
    pub fn get_env(&self) -> CallStack {
        self.env.clone()
    }
    fn push_env(&mut self, env: EnvReference) {
        self.env.push_env(env);
    }
    fn pop_env(&mut self) {
        self.env.pop_env();
    }
    fn current_env(&self) -> &EnvReference {
        self.env.current_env()
    }
    fn current_env_mut(&mut self) -> &mut EnvReference {
        self.env.current_env_mut()
    }

    fn set_value(&mut self, name: String, value: Reference) {
        let mut env = unsafe { self.current_env_mut().write().unwrap_unchecked() };
        env.insert(name, value);
    }

    fn get_value(&mut self, name: &str) -> Reference {
        for env in self.env.active_slice().iter().rev() {
            let map = unsafe { env.as_ref().read().unwrap_unchecked() };
            if let Some(value) = map.get(name) {
                return value.clone();
            }
        }

        return NULL.clone();
    }

    pub fn new(global_env: Env) -> Self {
        return Self {
            env: CallStack::new(EnvReference::new(EnvReferenceInner::new(global_env))),
        };
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
                let st = token.value.as_str();
                let mut table = NUMBER_LOOKUP_TABLE.lock().unwrap();

                if let Some(value) = table.get(st) {
                    return Ok(value.clone());
                } else {
                    let value = token
                        .value
                        .parse::<isize>()
                        .map(|v| Reference::new(Object::Integer(v)))
                        .context("error parsing numberliteral:")?;

                    table.insert(st.to_owned(), value.clone());

                    return Ok(value);
                }
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

                        self.push_env(EnvReference::new(EnvReferenceInner::new(
                            env.read().unwrap().clone(),
                        )));
                        for (idx, arg) in parameters.iter().enumerate() {
                            self.set_value(arg.clone(), args[idx].clone());
                        }
                        let ret = self.eval(&body)?;
                        self.pop_env();

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

                let env = self.current_env().clone();

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
