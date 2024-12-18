//! Builtin functions for working with lists
use std::sync::Arc;

use crate::interpreter::{
    map_rust_error,
    objects::{BuiltinFunction, Object},
    Env, EnvReference, EnvReferenceInner, Program, Reference, FUNCTION, LIST, NULL, NUMBER,
};

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

    env.insert(
        "sort".into(),
        Reference::new(Object::Builtin { function: sort }),
    );

    env.insert(
        "map".into(),
        Reference::new(Object::Builtin { function: MAP }),
    );
}

/// Maps a function over a list and returns it's results as a new list
pub const MAP: BuiltinFunction = |program, args| {
    let len = args.len();
    if len != 2 {
        return new_args_len_error("map", &args, 2);
    }

    let Object::List(l) = args[0].as_ref() else {
        return new_type_error_with_pos("map", LIST.type_of(), 0);
    };

    let Object::Function {
        parameters,
        body,
        env,
        ..
    } = args[1].as_ref()
    else {
        return new_type_error_with_pos("map", FUNCTION.type_of(), 1);
    };

    let base_env = env.read().clone();

    let result = l
        .iter()
        .map(|item| {
            let mut env = base_env.clone();

            if let Some(param) = parameters.first() {
                env.insert(param.clone(), item.clone());
            }

            program.push_env(EnvReference::new(EnvReferenceInner::new(env)));

            let result = program
                .parse_expression(body)
                .and_then(map_rust_error!("map error"));

            program.pop_env();

            result
        })
        .collect::<anyhow::Result<Arc<_>>>();

    match result {
        Ok(result) => Reference::new(Object::List(result)),
        Err(err) => Reference::new(Object::Error(err.to_string().into())),
    }
};

/// Returns the n-th element of a list
pub fn nth(_: &mut Program, args: Vec<Reference>) -> Reference {
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

    l.get(*n as usize).cloned().unwrap_or(NULL.clone())
}

/// Returns the first element of a list
pub fn head(_: &mut Program, args: Vec<Reference>) -> Reference {
    let len = args.len();
    if len != 1 {
        return new_args_len_error("head", &args, 1);
    }

    let Object::List(l) = args[0].as_ref() else {
        return new_type_error_with_pos("head", LIST.type_of(), 1);
    };

    if l.is_empty() {
        return NULL.clone();
    }

    let first = l.iter().next().cloned();

    first.unwrap_or_else(|| NULL.clone())
}

/// Returns the tail of a list
pub fn tail(_: &mut Program, args: Vec<Reference>) -> Reference {
    let len = args.len();
    if len != 1 {
        return new_args_len_error("tail", &args, 1);
    }

    let Object::List(l) = args[0].as_ref() else {
        return new_type_error_with_pos("tail", LIST.type_of(), 1);
    };

    let vec = l.iter().skip(1).cloned().collect();

    Reference::new(Object::List(vec))
}

/// Returns a slice of a list. If the third argument is not provided, the slice will go to the end of the list.
pub fn slice(_: &mut Program, args: Vec<Reference>) -> Reference {
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

    Reference::new(Object::List(vec))
}

pub fn sort(_: &mut Program, args: Vec<Reference>) -> Reference {
    let len = args.len();

    if len != 1 {
        return new_args_len_error("sort", &args, 2);
    }

    let Object::List(l) = args[0].as_ref() else {
        return new_type_error_with_pos("sort", LIST.type_of(), 0);
    };

    let mut vec = l.to_vec();
    vec.sort();

    Reference::new(Object::List(vec.into()))
}
