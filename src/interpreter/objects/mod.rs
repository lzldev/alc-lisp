use crate::ast::Node;

#[derive(Debug, Clone)]
pub enum Object {
    List(Vec<Object>),
    Integer(isize),
    String(String),
    Bool(bool),
    Builtin(fn(Vec<Object>) -> Object),
    Function { arguments: Vec<String>, body: Node },
    Null,
    Error(String),
}

unsafe impl Sync for Object {}
