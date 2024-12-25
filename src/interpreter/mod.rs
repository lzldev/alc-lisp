use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use foldhash::{HashMap, HashMapExt};
use objects::Object;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, RwLock};

use crate::ast::Node;

mod constants;
pub use constants::*;

pub mod builtins;
pub mod objects;

pub type Reference = Arc<Object>;
pub type EnvReference = Arc<EnvReferenceInner>;
pub type EnvReferenceInner = RwLock<Env>;
pub type Env = HashMap<Arc<str>, Reference>;

static NUMBER_LOOKUP_TABLE: Lazy<Mutex<HashMap<Arc<str>, Reference>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

macro_rules! map_rust_error {
    ($message:expr) => {
        |value: crate::interpreter::Reference| -> anyhow::Result<crate::interpreter::Reference> {
            if crate::interpreter::is_error(&value) {
                return Err(anyhow::anyhow!(concat!($message, ": {:?}"), value));
            }

            Ok(value)
        }
    };
}

pub(crate) use map_rust_error;

#[derive(Debug, Clone)]
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
            std::array::from_fn(|_| EnvReference::new(EnvReferenceInner::new(Env::default())));

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
        &self.stack[0..=self.sp]
    }
    pub fn active_slice_mut(&mut self) -> &mut [EnvReference] {
        &mut self.stack[0..=self.sp]
    }
}

impl Program {
    pub fn get_env(&self) -> CallStack {
        self.env.clone()
    }

    // fn push_env(&mut self, env: EnvReference) {
    //     self.env.push_env(env);
    // }

    // fn pop_env(&mut self) {
    //     self.env.pop_env();
    // }

    // fn current_env(&self) -> &EnvReference {
    //     self.env.current_env()
    // }

    fn current_env_mut(&mut self) -> &mut EnvReference {
        self.env.current_env_mut()
    }

    fn set_value(&mut self, name: Arc<str>, value: Reference) {
        let mut env = self.current_env_mut().write();
        env.insert(name, value);
    }

    pub fn run_function(
        &mut self,
        closure: &EnvReference,
        body: &Node,
        parameters: &Arc<[Arc<str>]>,
        args: &[Reference],
    ) -> anyhow::Result<Reference> {
        //TODO: When calling a function multiple times with the same parameters, the environment should be cloned only once
        let mut env = closure.read().clone();

        env.extend(parameters.iter().cloned().zip(args.iter().cloned()));

        self.env
            .push_env(EnvReference::new(EnvReferenceInner::new(env)));
        let result = self.eval(body);
        self.env.pop_env();

        result
    }

    fn get_value(&mut self, name: &str) -> Reference {
        for env in self.env.active_slice().iter().rev() {
            if let Some(value) = env.read().get(name) {
                return value.clone();
            }
        }

        NULL.clone()
    }

    pub fn new(global_env: Env) -> Self {
        Self {
            env: CallStack::new(EnvReference::new(EnvReferenceInner::new(global_env))),
        }
    }

    pub fn call_expression(&mut self, nodes: &[Node]) -> anyhow::Result<Reference> {
        if nodes.is_empty() {
            return Ok(NULL.clone());
        }

        let len = nodes.len();

        if let Node::Word(word) = &nodes[0] {
            match word.value.as_ref() {
                "define" | "def" => {
                    if len != 3 {
                        return Ok(Reference::new(Object::Error(
                            format!(
                                "Invalid amount of arguments to define got:{} expected: 3",
                                len
                            )
                            .into(),
                        )));
                    }

                    let name = match &nodes[1] {
                        Node::Word(token) => token,
                        n => {
                            return Ok(Reference::new(Object::Error(
                                format!("Invalid token for define: {:?} should be a word", n)
                                    .into(),
                            )))
                        }
                    };

                    let value = self
                        .parse_expression(&nodes[2])
                        .and_then(map_rust_error!("define value error"))?;

                    self.set_value(name.value.clone(), value);

                    return Ok(NULL.clone());
                }
                "if" => {
                    if len != 4 && len != 3 {
                        return Ok(Reference::new(Object::Error(
                            format!("Invalid amount of arguments to 'if' got: {}", len).into(),
                        )));
                    }

                    let condition = self
                        .parse_expression(&nodes[1])
                        .and_then(map_rust_error!("if condition error"))?;

                    let truthy = is_truthy(condition);

                    return if truthy {
                        self.parse_expression(&nodes[2])
                    } else if len == 4 {
                        self.parse_expression(&nodes[3])
                    } else {
                        Ok(NULL.clone())
                    }
                    .and_then(map_rust_error!("if result error"));
                }
                "do" => {
                    if len != 2 {
                        return Ok(Reference::new(Object::Error(
                            format!("Invalid amount of arguments to 'do' got: {}", len).into(),
                        )));
                    }

                    return self.eval(&nodes[1]);
                }
                _ => {}
            }
        }

        let first = self
            .parse_expression(&nodes[0])
            .and_then(map_rust_error!("in call to"))?;

        let args = nodes
            .iter()
            .skip(1)
            .map(|exp| {
                self.parse_expression(exp)
                    .and_then(map_rust_error!("function argument"))
            })
            .collect::<Result<Vec<_>>>()?;

        match first.as_ref() {
            Object::Builtin { function } => Ok(function(self, args)),
            Object::Function {
                env,
                parameters,
                body,
            } => {
                if args.len() != parameters.len() {
                    return Ok(Reference::new(Object::Error(
                        format!(
                            "Invalid number of arguments passed into function got {} expected {}",
                            args.len(),
                            parameters.len()
                        )
                        .into(),
                    )));
                }

                self.run_function(env, body, parameters, &args)
            }
            Object::Null => Ok(first),
            obj => Ok(Reference::new(Object::Error(
                format!("Cannot call value of type {}", obj.type_of()).into(),
            ))),
        }
    }

    pub fn eval(&mut self, root: &Node) -> anyhow::Result<Reference> {
        match root {
            Node::Expression(expressions) => {
                if !expressions.is_empty() && matches!(&expressions[0], Node::Word(_)) {
                    return self.call_expression(expressions);
                }

                if expressions.len() == 1 {
                    return self.parse_expression(&expressions[0]);
                }

                let mut last_result: Reference = NULL.clone();

                for exp in expressions.iter() {
                    last_result = self.parse_expression(exp).with_context(|| {
                        format!(
                            "error in expression at {}:{}",
                            exp.last_char().line,
                            exp.last_char().col,
                        )
                    })?;

                    if is_error(&last_result) {
                        return Err(anyhow!(
                            "error in expression at {}:{} : {:?}",
                            exp.last_char().line,
                            exp.last_char().col,
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
            Node::Word(token) => Ok(self.get_value(token.value.as_ref())),
            Node::BooleanLiteral(token) => match token.value.as_ref() {
                "true" => Ok(TRUE.clone()),
                "false" => Ok(FALSE.clone()),
                _ => panic!("This should never happen"),
            },
            Node::StringLiteral(token) => {
                let len = token.value.len();

                Ok(Reference::new(Object::String(
                    token.value[1..(len - 1)].into(),
                )))
            }
            Node::NumberLiteral(token) => {
                let mut table = NUMBER_LOOKUP_TABLE.lock();

                if let Some(value) = table.get(&token.value) {
                    Ok(value.clone())
                } else {
                    let value = token
                        .value
                        .parse::<isize>()
                        .map(|v| Reference::new(Object::Integer(v)))
                        .context("error parsing numberliteral:")?;

                    table.insert(token.value.clone(), value.clone());

                    Ok(value)
                }
            }
            Node::Invalid(_) => Ok(Reference::new(Object::Error(
                "Evaluating Invalid Node".into(),
            ))),
            Node::Expression(vec) => self.call_expression(vec),
            Node::List(vec) => {
                let items = vec
                    .iter()
                    .map(|item| {
                        self.parse_expression(item)
                            .and_then(map_rust_error!("list element"))
                    })
                    .collect::<Result<Arc<[_]>>>()?;

                Ok(Reference::new(Object::List(items)))
            }
            Node::FunctionLiteral {
                token: _,
                arguments,
                body,
            } => {
                let arguments = arguments
                    .iter()
                    .map(|arg| {
                        let Node::Word(token) = arg else {
                            return Err(anyhow!("argument is not a word"));
                        };

                        Ok(token.value.clone())
                    })
                    .collect::<Result<Arc<[_]>>>()?;

                let env = EnvReference::new(EnvReferenceInner::new(Env::with_capacity(16)));

                Ok(Reference::new(Object::Function {
                    env,
                    parameters: arguments,
                    body: (**body).clone(),
                }))
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
pub fn is_error(value: &Reference) -> bool {
    matches!(value.as_ref(), Object::Error(_))
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

    false
}
