use std::{
    borrow::BorrowMut,
    cell::{LazyCell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use anyhow::{anyhow, Context};
use objects::Object;

use crate::ast::Node;

pub mod builtins;
pub mod objects;

pub type Reference = Rc<Object>;
pub type Env = HashMap<String, Reference>;

pub struct Program {
    pub env: Vec<RefCell<Env>>,
}

//Sentinel Values
pub const TRUE: LazyCell<Reference> = LazyCell::new(|| Rc::new(Object::Bool(true)));
pub const FALSE: LazyCell<Reference> = LazyCell::new(|| Rc::new(Object::Bool(false)));
pub const NULL: LazyCell<Reference> = LazyCell::new(|| Rc::new(Object::Null));
pub const NUMBER: LazyCell<Reference> = LazyCell::new(|| Rc::new(Object::Integer(0)));
pub const STRING: LazyCell<Reference> = LazyCell::new(|| Rc::new(Object::String(String::new())));
pub const LIST: LazyCell<Reference> = LazyCell::new(|| Rc::new(Object::List(vec![])));

fn bool_from_native(value: bool) -> Reference {
    if value {
        TRUE.clone()
    } else {
        FALSE.clone()
    }
}

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

impl Program {
    pub fn new(global_env: Env) -> Self {
        return Self {
            env: vec![RefCell::new(global_env)],
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

    fn get_value(&mut self, name: &str) -> Rc<Object> {
        for env in self.env.iter_mut().rev() {
            let map = env.borrow_mut().get_mut();
            if let Some(value) = map.get(name) {
                return value.clone();
            }
        }

        return NULL.clone();
    }

    fn set_value(&mut self, name: String, value: Rc<Object>) {
        let env = self.env.last_mut().unwrap().borrow_mut().get_mut();
        env.insert(name, value);
    }

    fn parse_expression(&mut self, node: &Node) -> anyhow::Result<Reference> {
        match node {
            Node::Word(token) => Ok(self.get_value(token.value.as_str())),
            Node::BooleanLiteral(token) => match token.value.as_str() {
                "true" => Ok(TRUE.clone()),
                "false" => Ok(FALSE.clone()),
                _ => panic!("AAAAA"),
            },
            Node::StringLiteral(token) => {
                let len = token.value.len();

                return Ok(Rc::new(Object::String(
                    token.value[1..(len - 1)].to_owned(),
                )));
            }
            Node::NumberLiteral(token) => {
                let value = token
                    .value
                    .parse::<isize>()
                    .context("error parsing numberliteral:")?;

                return Ok(Rc::new(Object::Integer(value)));
            }
            Node::Invalid(_) => {
                return Ok(Rc::new(Object::Error("Evaluating Invalid Node".to_owned())))
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
                                return Ok(Rc::new(Object::Error(format!(
                                    "Invalid amount of arguments to define got:{} expected: 3",
                                    len
                                ))));
                            }

                            let name = match &vec[1] {
                                Node::Word(token) => token,
                                n => {
                                    return Ok(Rc::new(Object::Error(format!(
                                        "Invalid token for define: {:?} should be a word",
                                        n
                                    ))))
                                }
                            };

                            let value = self.parse_expression(&vec[2])?;

                            if is_error(&value) {
                                return Err(anyhow!("define value error: {:?}", value));
                            }

                            self.set_value(name.value.clone(), value);

                            return Ok(NULL.clone());
                        }
                        "if" => {
                            if len != 4 && len != 3 {
                                return Ok(Rc::new(Object::Error(format!(
                                    "Invalid amount of arguments to if got: {}",
                                    len
                                ))));
                            }

                            let condition = self.parse_expression(&vec[1])?;

                            if is_error(&condition) {
                                return Err(anyhow!("if condition error: {:?}", condition));
                            }

                            let truthy = is_truthy(condition);

                            let result = if truthy {
                                self.parse_expression(&vec[2])?
                            } else if len == 4 {
                                self.parse_expression(&vec[3])?
                            } else {
                                NULL.clone()
                            };

                            if is_error(&result) {
                                return Err(anyhow!("if result error: {:?}", result));
                            }

                            return Ok(result);
                        }
                        "do" => {
                            if len != 2 {
                                return Ok(Rc::new(Object::Error(format!(
                                    "Invalid amount of arguments to do got: {}",
                                    len
                                ))));
                            }

                            return self.eval(&vec[1]);
                        }
                        _ => {}
                    }
                }

                let first = self.parse_expression(&vec[0])?;

                if is_error(&first) {
                    return Err(anyhow!("error in call to: {:?}", first));
                }

                if len == 1 {
                    return Ok(first);
                }

                let mut args: Vec<Reference> = Vec::with_capacity(len - 1);

                for exp in &vec[1..] {
                    let value = self.parse_expression(exp)?;

                    if is_error(&value) {
                        return Err(anyhow!("error in function argument: {:?}", value));
                    }

                    args.push(value);
                }

                match first.as_ref() {
                    Object::Builtin { function } => return Ok(function(args)),
                    Object::Function {
                        env,
                        parameters,
                        body,
                    } => {
                        if args.len() != parameters.len() {
                            return Ok(Rc::new(Object::Error(format!("Invalid number of arguments passed into function got {} expected {}",args.len(),parameters.len()))));
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
                let mut items: Vec<Reference> = Vec::with_capacity(vec.len());

                for item in vec.iter() {
                    let value = self.parse_expression(item)?;

                    if is_error(&value) {
                        return Err(anyhow!("error in list item: {:?}", value));
                    }

                    items.push(value);
                }

                return Ok(Rc::new(Object::List(items)));
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
                            panic!("argument is not a word")
                        };
                        return token.value.clone();
                    })
                    .collect();

                let env = self.env.last().expect("to get last env").clone();

                return Ok(Rc::new(Object::Function {
                    env,
                    parameters: arguments,
                    body: (**body).clone(),
                }));
            }
        }
    }
}
