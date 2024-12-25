use crate::interpreter::{objects::Object, Reference};

pub fn new_args_len_error(name: &str, args: &[Reference], expected: usize) -> Reference {
    Reference::new(Object::Error(
        format!(
            "Invalid amount of argument to function '{}': expected: {} got: {}",
            name,
            expected,
            args.len(),
        )
        .into(),
    ))
}

pub fn new_type_error(name: &str, typename: &str) -> Reference {
    Reference::new(Object::Error(
        format!(
            "Invalid argument type for function '{}': expected {}",
            name, typename
        )
        .into(),
    ))
}

pub fn new_type_error_with_got(name: &str, typename: &str, gottype: &str) -> Reference {
    Reference::new(Object::Error(
        format!(
            "Invalid argument type for function '{}': expected {} got {}",
            name, typename, gottype
        )
        .into(),
    ))
}

pub fn new_type_error_with_got_and_pos(
    name: &str,
    pos: usize,
    typename: &str,
    gottype: &str,
) -> Reference {
    Reference::new(Object::Error(
        format!(
            "Invalid argument type for function '{}' at position {} expected {} got {}",
            name, pos, typename, gottype
        )
        .into(),
    ))
}

pub fn new_type_error_with_pos(name: &str, typename: &str, pos: usize) -> Reference {
    Reference::new(Object::Error(
        format!(
            "Invalid argument type for function '{}': expected {} at position {}",
            name, typename, pos
        )
        .into(),
    ))
}
