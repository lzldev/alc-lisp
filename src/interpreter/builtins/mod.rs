//! Generic builtin functions
pub mod errors;
mod list;
mod number;
mod string;

#[cfg(feature = "bin")]
pub mod native;

use errors::{new_args_len_error, new_type_error};
use list::add_list_builtins;
use number::add_number_builtins;
use string::add_string_builtins;

use super::{
    bool_from_native,
    objects::{BuiltinFunction, Object},
    Env, Program, Reference, LIST, STRING, TRUE,
};

fn typecheck_args<F>(
    name: &str,
    typename: &str,
    condition: F,
    args: &[Reference],
) -> Option<Reference>
where
    F: Fn(&Reference) -> bool,
{
    if args.iter().any(condition) {
        return Some(new_type_error(name, typename));
    };
    None
}

/// Adds all builtin functions to the environment
pub fn add_generic_builtins(env: &mut Env) {
    add_number_builtins(env);
    add_list_builtins(env);
    add_string_builtins(env);

    env.insert(
        "len".into(),
        Reference::new(Object::Builtin { function: LEN }),
    );

    env.insert(
        "==".into(),
        Reference::new(Object::Builtin { function: EQUALS }),
    );

    env.insert(
        "!=".into(),
        Reference::new(Object::Builtin {
            function: NOT_EQUALS,
        }),
    );

    env.insert(
        "<".into(),
        Reference::new(Object::Builtin {
            function: LESSER_THAN,
        }),
    );

    env.insert(
        ">".into(),
        Reference::new(Object::Builtin {
            function: GREATHER_THAN,
        }),
    );
}

/// Returns the length of a list or string
const LEN: BuiltinFunction = |_: &mut Program, args: Vec<Reference>| -> Reference {
    if args.len() != 1 {
        return new_args_len_error("len", &args, 1);
    }

    match args[0].as_ref() {
        Object::String(s) => Reference::new(Object::Integer(s.len() as isize)),
        Object::List(l) => Reference::new(Object::Integer(l.len() as isize)),
        _ => new_type_error(
            "len",
            &format!("{} or {}", STRING.type_of(), LIST.type_of()),
        ),
    }
};

const NOT_EQUALS: BuiltinFunction = |_: &mut Program, args: Vec<Reference>| -> Reference {
    let len = args.len();
    if len == 0 {
        return new_args_len_error("==", &args, 2);
    }

    if len == 1 {
        return TRUE.clone();
    }

    let mut first = &args[0];

    for last in args.iter().skip(1) {
        let value = first.as_ref() == last.as_ref();

        if !value {
            return bool_from_native(true);
        }

        first = last;
    }

    bool_from_native(false)
};

/// Equal comparison between values
const EQUALS: BuiltinFunction = |_: &mut Program, args: Vec<Reference>| -> Reference {
    let len = args.len();
    if len == 0 {
        return new_args_len_error("==", &args, 2);
    }

    if len == 1 {
        return TRUE.clone();
    }

    let mut first = &args[0];

    for last in args.iter().skip(1) {
        let value = first.as_ref() == last.as_ref();

        if !value {
            return bool_from_native(false);
        }

        first = last;
    }

    bool_from_native(true)
};

/// Lesser than comparison between values
const LESSER_THAN: BuiltinFunction = |_: &mut Program, args: Vec<Reference>| -> Reference {
    let len = args.len();
    if len == 0 {
        return new_args_len_error("<", &args, 2);
    }

    if len == 1 {
        return TRUE.clone();
    }

    let mut first = &args[0];

    for last in args.iter().skip(1) {
        let value = match (first.as_ref(), last.as_ref()) {
            (Object::Integer(l), Object::Integer(r)) => l < r,
            _ => false,
        };

        if !value {
            return bool_from_native(false);
        }

        first = last;
    }

    bool_from_native(true)
};

/// Greater than comparison between values
const GREATHER_THAN: BuiltinFunction = |_: &mut Program, args: Vec<Reference>| -> Reference {
    let len = args.len();
    if len == 0 {
        return new_args_len_error(">", &args, 2);
    }

    if len == 1 {
        return TRUE.clone();
    }

    let mut first = &args[0];

    for last in args.iter().skip(1) {
        let value = match (first.as_ref(), last.as_ref()) {
            (Object::Integer(l), Object::Integer(r)) => l > r,
            _ => false,
        };

        if !value {
            return bool_from_native(false);
        }

        first = last;
    }

    bool_from_native(true)
};
