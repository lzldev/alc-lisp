use std::{cell::RefCell, fmt::Display};

use crate::ast::Node;

use super::{Env, Reference};

#[derive(Debug, Clone)]
pub enum Object {
    Null,
    Integer(isize),
    String(String),
    Bool(bool),
    List(Vec<Reference>),
    Builtin(fn(Vec<Reference>) -> Reference),
    Function {
        env: RefCell<Env>,
        parameters: Vec<String>,
        body: Node,
    },
    Error(String),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => f.write_str("null"),
            Object::Integer(v) => write!(f, "{}", v),
            Object::String(v) => f.write_str(v.as_str()),
            Object::Bool(v) => write!(f, "{}", v),
            Object::List(vec) => {
                f.write_str("[")?;
                for v in vec.iter() {
                    v.fmt(f)?;
                    f.write_str(" ")?;
                }
                f.write_str("]")?;
                Ok(())
            }
            Object::Builtin(v) => {
                write!(f, "{:?}", v)
            }
            Object::Function { .. } => {
                write!(f, "{:p}", self)
            }
            Object::Error(msg) => {
                write!(f, "Error:{}", msg)
            }
        }
    }
}
