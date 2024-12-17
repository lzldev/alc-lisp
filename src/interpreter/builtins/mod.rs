mod errors;
mod list;
mod number;
mod string;

use errors::{new_args_len_error, new_type_error};
use list::add_list_builtins;
use number::add_number_builtins;
use string::add_string_builtins;

use crate::interpreter::NULL;

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
        Reference::new(Object::Builtin {
            function: |args| {
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
            },
        }),
    );

    env.insert(
        "==".into(),
        Reference::new(Object::Builtin {
            function: |args| {
                let len = args.len();
                if len == 0 {
                    return new_args_len_error("==", &args, 2);
                }

                if len == 1 {
                    return TRUE.clone();
                }

                let mut first = &args[0];

                for last in args.iter().skip(1) {
                    let value = match (first.as_ref(), last.as_ref()) {
                        (Object::Null, Object::Null) => true,
                        (Object::Bool(l), Object::Bool(r)) => l == r,
                        (Object::Integer(l), Object::Integer(r)) => l == r,
                        (Object::String(l), Object::String(r)) => l == r,
                        _ => false,
                    };

                    if !value {
                        return bool_from_native(false);
                    }

                    first = last;
                }

                return bool_from_native(true);
            },
        }),
    );

    env.insert(
        "<".into(),
        Reference::new(Object::Builtin {
            function: |args| {
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
            },
        }),
    );

    env.insert(
        ">".into(),
        Reference::new(Object::Builtin {
            function: |args| {
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
            },
        }),
    );
}

pub fn add_native_builtins(env: &mut Env) {
    env.insert(
        "print".into(),
        Reference::new(Object::Builtin {
            function: |args| {
                println!("{}", args.iter().map(|v| v.to_string()).collect::<String>());
                return NULL.clone();
            },
        }),
    );

    env.insert(
        "debug".into(),
        Reference::new(Object::Builtin {
            function: |args| {
                println!("{:?}", args);
                return NULL.clone();
            },
        }),
    );
}
