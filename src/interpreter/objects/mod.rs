#[derive(Debug, Clone)]
pub enum Object {
    List(Vec<Object>),
    Integer(usize),
    String(String),
    Bool(bool),
    Builtin(fn(Vec<Object>) -> Object),
    Function,
    Null,
    Error(String),
}

unsafe impl Sync for Object {}
