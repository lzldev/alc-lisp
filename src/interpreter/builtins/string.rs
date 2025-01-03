//! Builtin functions for string operations
use std::sync::Arc;

use crate::interpreter::{
    objects::{BuiltinFunction, Object},
    Env, Reference, STRING,
};

use super::{
    errors::{new_args_len_error, new_type_error_with_pos},
    type_check, typecheck_args,
};

pub fn add_string_builtins(env: &mut Env) {
    let functions: [(&str, BuiltinFunction); _] =
        [("str", STR), ("lines", LINES), ("split", SPLIT)];

    functions
        .into_iter()
        .map(|(name, function)| (name, Reference::new(Object::Builtin { function })))
        .for_each(|(name, function)| {
            env.insert(name.into(), function.clone());
            env.insert(("std/".to_owned() + name).into(), function);
        });
}

/// Concatenates the arguments into a string
pub const STR: BuiltinFunction = |_, args| {
    type_check!("str", args, Object::String(_));

    let result = args
        .iter()
        .map(|v| {
            let Object::String(inner) = v.as_ref() else {
                panic!("This should never happen");
            };

            inner.as_ref()
        })
        .collect::<String>();

    Reference::new(Object::String(result.into()))
};

/// Splits a string into a list of lines
pub const LINES: BuiltinFunction = |_, args| {
    let len = args.len();
    if len != 1 {
        return new_args_len_error("lines", &args, 1);
    }

    if let Some(err) = typecheck_args(
        "lines",
        STRING.type_of(),
        |obj| !matches!(obj.as_ref(), Object::String(_)),
        &args,
    ) {
        return err;
    }

    let Object::String(inner) = args[0].as_ref() else {
        panic!("This should never happen");
    };

    let lines = inner
        .lines()
        .map(|v| Reference::new(Object::String(v.into())))
        .collect::<Arc<_>>();

    Reference::new(Object::List(lines))
};

/// Splits a string into a list based on the delimiter
pub const SPLIT: BuiltinFunction = |_, args| {
    let len = args.len();
    if len != 2 {
        return new_args_len_error("split", &args, 2);
    }

    let Object::String(input) = args[0].as_ref() else {
        return new_type_error_with_pos("split", STRING.type_of(), 0);
    };

    let Object::String(split) = args[1].as_ref() else {
        return new_type_error_with_pos("split", STRING.type_of(), 1);
    };

    let list = input
        .split(split.as_ref())
        .map(|v| Reference::new(Object::String(v.into())))
        .collect::<Arc<_>>();

    Reference::new(Object::List(list))
};
