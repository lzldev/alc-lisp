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

use super::{bool_from_native, objects::Object, Env, Reference, LIST, STRING, TRUE};

fn typecheck_args<F>(
    name: &str,
    typename: &str,
    condition: F,
    args: &Vec<Reference>,
) -> Option<Reference>
where
    F: Fn(&Reference) -> bool,
{
    if args.iter().any(condition) {
        return Some(new_type_error(name, typename));
    };
    None
}

pub fn add_generic_builtins(env: &mut Env) {
    add_number_builtins(env);
    add_list_builtins(env);
    add_string_builtins(env);

    env.insert(
        "len".into(),
        Reference::new(Object::Builtin { function: len }),
    );

    env.insert(
        "==".into(),
        Reference::new(Object::Builtin { function: equals }),
    );

    env.insert(
        "<".into(),
        Reference::new(Object::Builtin {
            function: lesser_than,
        }),
    );

    env.insert(
        ">".into(),
        Reference::new(Object::Builtin {
            function: greather_than,
        }),
    );
}

pub fn len(args: Vec<Reference>) -> Reference {
    if args.len() != 1 {
        return new_args_len_error("len", &args, 1);
    }

    match args[0].as_ref() {
        Object::String(s) => return Reference::new(Object::Integer(s.len() as isize)),
        Object::List(l) => return Reference::new(Object::Integer(l.len() as isize)),
        _ => {
            return new_type_error(
                "len",
                &format!("{} or {}", STRING.type_of(), LIST.type_of()),
            )
        }
    }
}

pub fn equals(args: Vec<Reference>) -> Reference {
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

    return bool_from_native(true);
}

pub fn lesser_than(args: Vec<Reference>) -> Reference {
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

    return bool_from_native(true);
}

pub fn greather_than(args: Vec<Reference>) -> Reference {
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

    return bool_from_native(true);
}
