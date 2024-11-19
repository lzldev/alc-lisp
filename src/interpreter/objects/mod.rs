use crate::ast::Node;

use super::Reference;

#[derive(Debug, Clone)]
pub enum Object {
    List(Vec<Reference>),
    Integer(isize),
    String(String),
    Bool(bool),
    Builtin(fn(Vec<Reference>) -> Reference),
    Function { parameters: Vec<String>, body: Node },
    Null,
    Error(String),
}

unsafe impl Sync for Object {}
