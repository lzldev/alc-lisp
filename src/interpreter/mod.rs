use std::{
    borrow::BorrowMut,
    cell::{Cell, LazyCell},
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
    pub env: Vec<Cell<Env>>,
}

pub const TRUE: LazyCell<Reference> = LazyCell::new(|| Rc::new(Object::Bool(true)));
pub const FALSE: LazyCell<Reference> = LazyCell::new(|| Rc::new(Object::Bool(false)));
pub const NULL: LazyCell<Reference> = LazyCell::new(|| Rc::new(Object::Null));

fn bool_from_native(value: bool) -> Reference {
    if value {
        TRUE.clone()
    } else {
        FALSE.clone()
    }
}

impl Program {
    pub fn new(global_env: Env) -> Self {
        return Self {
            env: vec![Cell::new(global_env)],
        };
    }

    pub fn eval(&mut self, root: &Node) -> anyhow::Result<Reference> {
        let mut last_result: Reference = NULL.clone();

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
            Node::BooleanLiteral(token) => match token.value.as_str() {
                "true" => Ok(TRUE.clone()),
                "false" => Ok(FALSE.clone()),
                _ => panic!("AAAAA"),
            },
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
                        "define" => {
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

                            self.set_value(name.value.clone(), value);

                            return Ok(NULL.clone());
                        }
                        _ => {}
                    }
                }

                let first = self.parse_expression(&vec[0])?;

                if len == 1 {
                    return Ok(first);
                }

                let mut args: Vec<Reference> = Vec::with_capacity(len - 1);

                for exp in &vec[1..] {
                    args.push(self.parse_expression(exp)?);
                }

                match first.as_ref() {
                    Object::Builtin(f) => return Ok(f(args)),
                    Object::Function { parameters, body } => {
                        if args.len() != parameters.len() {
                            return Ok(Rc::new(Object::Error(format!("Invalid number of arguments passed into function got {} expected {}",args.len(),parameters.len()))));
                        }

                        self.env.push(Cell::new(HashMap::new()));
                        for (idx, arg) in parameters.iter().enumerate() {
                            self.set_value(arg.clone(), args[idx].clone());
                        }
                        let ret = self.parse_expression(&body)?;
                        self.env.pop();

                        return Ok(ret);
                    }
                    obj => return Err(anyhow!("Invalid type starting expression {:?}", obj)),
                }
            }
            Node::List(vec) => {
                let mut items: Vec<Reference> = Vec::with_capacity(vec.len());

                for item in vec.iter() {
                    items.push(self.parse_expression(item)?);
                }

                return Ok(Rc::new(Object::List(items)));
            }
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

                return Ok(Rc::new(Object::Function {
                    parameters: arguments,
                    body: (**body).clone(),
                }));
            }
            Node::Word(token) => Ok(self.get_value(token.value.as_str())),
        }
    }
}
