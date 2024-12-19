use std::{
    fmt::Display,
    sync::{Arc, LazyLock},
};

use crate::ast::Node;

use super::{EnvReference, Program, Reference, NULL};

#[cfg(feature = "wasm")]
mod wasm;

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
    String(Arc<str>),
    Bool(bool),
    List(Arc<[Reference]>),
    Builtin {
        #[cfg_attr(feature = "serde", serde(default = "get_default_builtin", skip))]
        function: BuiltinFunction,
    },
    Function {
        #[cfg_attr(feature = "ts-rs", ts(skip))]
        env: EnvReference,
        parameters: Arc<[Arc<str>]>,
        body: Node,
    },
    Error(Arc<str>),
}

pub type BuiltinFunction = fn(&mut Program, Vec<Reference>) -> Reference;

pub static DEFAULT_BUILTIN: LazyLock<BuiltinFunction> =
    LazyLock::new(|| |_: &mut Program, _: Vec<Reference>| -> Reference { NULL.clone() });

#[cfg(feature = "serde")]
fn get_default_builtin() -> fn(&mut Program, Vec<Reference>) -> Reference {
    *DEFAULT_BUILTIN
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
            Object::String(v) => f.write_fmt(format_args!("\"{}\"", v)),
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

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (
                Self::Builtin {
                    function: l_function,
                },
                Self::Builtin {
                    function: r_function,
                },
            ) => std::ptr::addr_eq(l_function, r_function),
            (
                Self::Function {
                    env: l_env,
                    parameters: l_parameters,
                    body: l_body,
                },
                Self::Function {
                    env: r_env,
                    parameters: r_parameters,
                    body: r_body,
                },
            ) => {
                std::ptr::addr_eq(l_env, r_env)
                    && std::ptr::addr_eq(l_parameters, r_parameters)
                    && std::ptr::addr_eq(l_body, r_body)
            }
            (Self::Error(l0), Self::Error(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
impl Eq for Object {}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Object {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Object::Null, _) => std::cmp::Ordering::Less,
            (Object::Bool(left), Object::Bool(right)) => left.cmp(right),
            (Object::Bool(_), _) => std::cmp::Ordering::Less,
            (Object::Integer(left), Object::Integer(right)) => left.cmp(right),
            (Object::Integer(_), _) => std::cmp::Ordering::Less,
            (Object::String(left), Object::String(right)) => left.cmp(right),
            (Object::String(_), _) => std::cmp::Ordering::Less,
            (Object::List(left), Object::List(right)) => {
                let llen = left.len();
                let rlen = right.len();

                llen.cmp(&rlen)
            }
            (Object::List(_), _) => std::cmp::Ordering::Less,
            (Object::Builtin { .. }, _) => std::cmp::Ordering::Less,
            (Object::Function { .. }, _) => std::cmp::Ordering::Less,
            (Object::Error { .. }, _) => std::cmp::Ordering::Less,
        }
    }
}
