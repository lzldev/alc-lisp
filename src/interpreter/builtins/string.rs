//! Builtin functions for string operations
use std::rc::Rc;

use crate::interpreter::{objects::Object, Env, Reference, STRING};

use super::{errors::new_args_len_error, typecheck_args};

pub fn add_string_builtins(env: &mut Env) {
    env.insert("str".into(), Rc::new(Object::Builtin { function: str }));
    env.insert("lines".into(), Rc::new(Object::Builtin { function: lines }));
}

/// Concatenates the arguments into a string
pub fn str(args: Vec<Reference>) -> Reference {
    if let Some(err) = typecheck_args(
        "str",
        STRING.type_of(),
        |obj| !matches!(obj.as_ref(), Object::String(_)),
        &args,
    ) {
        return err;
    }

    let result = args
        .iter()
        .map(|v| {
            let Object::String(inner) = v.as_ref() else {
                panic!("This should never happen");
            };

            inner
        })
        .cloned()
        .collect::<String>();

    Rc::new(Object::String(result))
}

/// Splits a string into a list of lines
fn lines(args: Vec<Reference>) -> Reference {
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
        .split("\n")
        .map(|v| Reference::new(Object::String(v.to_owned())))
        .collect::<Vec<_>>();

    Rc::new(Object::List(lines))
}
