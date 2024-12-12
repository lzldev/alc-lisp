use std::{
    cell::{LazyCell, RefCell},
    fmt::Display,
};

use js_sys::JsString;
use wasm_bindgen::JsValue;

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

#[cfg(feature = "wasm")]
pub const BUILTIN_MESSAGE: LazyCell<wasm_bindgen::JsValue> =
    LazyCell::new(|| JsString::from("Builtin Function").into());

#[cfg(feature = "wasm")]
impl From<Object> for wasm_bindgen::JsValue {
    fn from(value: Object) -> Self {
        match value {
            Object::Null => wasm_bindgen::JsValue::NULL,
            Object::Integer(value) => wasm_bindgen::JsValue::from(value as i32),
            Object::String(st) => JsString::from(st).into(),
            Object::Bool(value) => {
                if value {
                    wasm_bindgen::JsValue::TRUE
                } else {
                    wasm_bindgen::JsValue::FALSE
                }
            }
            Object::List(vec) => {
                let array = js_sys::Array::new_with_length(vec.len() as u32);

                vec.into_iter().enumerate().for_each(|(idx, arg)| {
                    array.set(idx as u32, JsValue::from((*arg).clone()));
                });

                array.into()
            }
            Object::Builtin { .. } => BUILTIN_MESSAGE.clone(),
            Object::Function { env, .. } => {
                JsString::from(format!("FUNCTION [{:p}]", env.as_ptr())).into()
            }
            Object::Error(_) => todo!(),
        }
    }
}
