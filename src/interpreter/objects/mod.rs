use std::{
    cell::{LazyCell, RefCell},
    fmt::Display,
};

use crate::ast::Node;

use super::{Env, Reference, NULL};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", content = "value")
)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS), ts(export))]
pub enum Object {
    Null,
    Integer(isize),
    String(String),
    Bool(bool),
    List(Vec<Reference>),
    Builtin {
        #[serde(default = "get_default_builtin", skip)]
        function: BuiltinFunction,
    },
    Function {
        env: RefCell<Env>,
        parameters: Vec<String>,
        body: Node,
    },
    Error(String),
}

type BuiltinFunction = fn(Vec<Reference>) -> Reference;

pub const DEFAULT_BUILTIN: LazyCell<BuiltinFunction> =
    LazyCell::new(|| |_: Vec<Reference>| -> Reference { NULL.clone() });

fn get_default_builtin() -> fn(Vec<Reference>) -> Reference {
    DEFAULT_BUILTIN.clone()
}

impl Object {
    pub fn type_of(&self) -> &'static str {
        match self {
            Object::Null => "null",
            Object::Integer(_) => "int",
            Object::String(_) => "string",
            Object::Bool(_) => "bool",
            Object::List(_) => "list",
            Object::Builtin { .. } => "builtin",
            Object::Function { .. } => "function",
            Object::Error(_) => "error",
        }
    }
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
                let len = vec.len();
                for (i, v) in vec.iter().enumerate() {
                    v.fmt(f)?;
                    if i != (len - 1) {
                        f.write_str(" ")?;
                    }
                }
                f.write_str("]")?;
                Ok(())
            }
            Object::Builtin { function } => {
                write!(f, "BUILTIN[{:?}]", function)
            }
            Object::Function { .. } => {
                write!(f, "FUNCTION[{:p}]", self)
            }
            Object::Error(msg) => {
                write!(f, "Error:{}", msg)
            }
        }
    }
}
