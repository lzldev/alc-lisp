use std::cell::RefCell;

use crate::ast::Node;

use super::{Env, Reference};

#[derive(Debug, Clone)]
pub enum Object {
    List(Vec<Reference>),
    Integer(isize),
    String(String),
    Bool(bool),
    Builtin(fn(Vec<Reference>) -> Reference),
    Function {
        env: RefCell<Env>,
        parameters: Vec<String>,
        body: Node,
    },
    Null,
    Error(String),
}

unsafe impl Sync for Object {}
