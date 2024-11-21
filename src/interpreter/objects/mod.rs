use std::cell::RefCell;

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
