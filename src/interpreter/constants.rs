use std::cell::LazyCell;

use super::{objects::Object, Reference};

pub const TRUE: LazyCell<Reference> = LazyCell::new(|| Reference::new(Object::Bool(true)));
pub const FALSE: LazyCell<Reference> = LazyCell::new(|| Reference::new(Object::Bool(false)));
pub const NULL: LazyCell<Reference> = LazyCell::new(|| Reference::new(Object::Null));
pub const NUMBER: LazyCell<Reference> = LazyCell::new(|| Reference::new(Object::Integer(0)));
pub const STRING: LazyCell<Reference> =
    LazyCell::new(|| Reference::new(Object::String(String::new())));
pub const LIST: LazyCell<Reference> = LazyCell::new(|| Reference::new(Object::List(vec![])));
