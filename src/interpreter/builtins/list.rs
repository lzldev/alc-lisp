//! Builtin functions for working with lists
use crate::interpreter::{objects::Object, Env, Reference, LIST, NULL, NUMBER};

use super::errors::{new_args_len_error, new_type_error_with_pos};

pub fn add_list_builtins(env: &mut Env) {
    env.insert(
        "nth".into(),
        Reference::new(Object::Builtin { function: nth }),
    );

    env.insert(
        "head".into(),
        Reference::new(Object::Builtin { function: head }),
    );

    env.insert(
        "tail".into(),
        Reference::new(Object::Builtin { function: tail }),
    );

    env.insert(
        "slice".into(),
        Reference::new(Object::Builtin { function: slice }),
    );
}

/// Returns the n-th element of a list
pub fn nth(args: Vec<Reference>) -> Reference {
    let len = args.len();
    if len != 2 {
        return new_args_len_error("nth", &args, 2);
    }

    let Object::Integer(n) = args[0].as_ref() else {
        return new_type_error_with_pos("nth", NUMBER.type_of(), 1);
    };
    let Object::List(l) = args[1].as_ref() else {
        return new_type_error_with_pos("nth", LIST.type_of(), 2);
    };

    return l
        .get(*n as usize)
        .map(|v| v.clone())
        .unwrap_or(NULL.clone());
}

/// Returns the first element of a list
pub fn head(args: Vec<Reference>) -> Reference {
    let len = args.len();
    if len != 1 {
        return new_args_len_error("head", &args, 1);
    }

    let Object::List(l) = args[0].as_ref() else {
        return new_type_error_with_pos("head", LIST.type_of(), 1);
    };

    if l.len() == 0 {
        return NULL.clone();
    }

    let first = l.iter().cloned().next();

    return first.unwrap_or_else(|| NULL.clone());
}

/// Returns the tail of a list
pub fn tail(args: Vec<Reference>) -> Reference {
    let len = args.len();
    if len != 1 {
        return new_args_len_error("tail", &args, 1);
    }

    let Object::List(l) = args[0].as_ref() else {
        return new_type_error_with_pos("tail", LIST.type_of(), 1);
    };

    let vec = l.iter().skip(1).cloned().collect();

    return Reference::new(Object::List(vec));
}

/// Returns a slice of a list. If the third argument is not provided, the slice will go to the end of the list.
pub fn slice(args: Vec<Reference>) -> Reference {
    let len = args.len();
    if len != 2 && len != 3 {
        return new_args_len_error("slice", &args, 2);
    }

    let Object::List(l) = args[0].as_ref() else {
        return new_type_error_with_pos("slice", LIST.type_of(), 0);
    };
    let Object::Integer(n) = args[1].as_ref() else {
        return new_type_error_with_pos("slice", NUMBER.type_of(), 1);
    };

    let end = if let Some(Object::Integer(end)) = args.get(2).map(|v| v.as_ref()) {
        *end as usize
    } else {
        l.len()
    };

    let vec = l
        .iter()
        .skip(*n as usize)
        .take(end - *n as usize)
        .cloned()
        .collect();

    return Reference::new(Object::List(vec));
}
