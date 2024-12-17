use crate::interpreter::{objects::Object, Reference};

pub fn new_args_len_error(name: &str, args: &Vec<Reference>, expected: usize) -> Reference {
    Reference::new(Object::Error(format!(
        "Invalid argument type for function '{}': got: {} expected: {}",
        name,
        args.len(),
        expected
    )))
}

pub fn new_type_error(name: &str, typename: &str) -> Reference {
    Reference::new(Object::Error(format!(
        "Invalid argument type for function '{}': expected {}",
        name, typename
    )))
}

pub fn new_type_error_with_pos(name: &str, typename: &str, pos: usize) -> Reference {
    Reference::new(Object::Error(format!(
        "Invalid argument type for function '{}': expected {} at position {}",
        name, typename, pos
    )))
}
