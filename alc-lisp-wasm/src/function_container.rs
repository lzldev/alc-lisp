use std::hash::Hash;

use js_sys::{Function, Object};

#[derive(Clone, Debug)]
pub struct FunctionContainer {
    pub function: Function,
}

impl Hash for FunctionContainer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Into::<String>::into(self.function.to_string()).hash(state)
    }
}

impl PartialEq for FunctionContainer {
    fn eq(&self, other: &Self) -> bool {
        Object::is(&self.function, &other.function)
    }
}

impl Eq for FunctionContainer {}

unsafe impl Sync for FunctionContainer {}
unsafe impl Send for FunctionContainer {}
