//! Generic builtin functions
pub mod errors;
mod list;
mod number;
mod string;

#[cfg(feature = "bin")]
pub mod native;

use errors::{new_args_len_error, new_type_error_with_got};
use list::add_list_builtins;
use number::add_number_builtins;
use string::add_string_builtins;

use super::{
    bool_from_native,
    objects::{BuiltinFunction, Object},
    Env, Program, Reference, NULL, TRUE,
};

#[allow(unused_macros)]
macro_rules! unwrap_args {
    ($args:ident,[$($ty:pat),+]) => {
        $(
            let $ty = $args[${index()}].as_ref() else {
                unreachable!()
            };
        )*
    };
}
pub(crate) use unwrap_args;

#[allow(unused_macros)]
macro_rules! unsafe_unwrap_args {
    ($args:ident,[$($ty:pat),+]) => {
        $(
            let $ty = $args[${index()}].as_ref() else {
                unsafe {std::hint::unreachable_unchecked()}
            };
        )*
    };
}
pub(crate) use unsafe_unwrap_args;

#[allow(unused_macros)]
macro_rules! type_check {
    ($name:expr,$args:ident,[$($ty:pat),+]) => {
        {
            let count = ${count($ty)};

            if $args.len() > count {
                return crate::interpreter::builtins::errors::new_args_len_error($name, &$args, count);
            }

            $(
                let arg = unsafe { $args.get(${index()}).unwrap_unchecked()};

                if !matches!(arg.as_ref(), $ty) {
                    let type_name = crate::interpreter::constants::ALL_TYPES
                    .iter()
                    .filter(|v| matches!(v.as_ref(), $ty))
                    .map(|v| v.type_of())
                    .collect::<std::sync::Arc<[_]>>()
                    .join(" or ");

                    return crate::interpreter::builtins::errors::new_type_error_with_got_and_pos(
                        $name,
                        ${index()},
                        &type_name,
                        arg.type_of(),
                    );
                }
            )*
        }
    };

    // for variadic functions
    ($name:expr,$args:ident,$type:pat) => {
        for (i,arg) in $args.iter().enumerate() {
            if !matches!(arg.as_ref(), $type) {
                let type_name = crate::interpreter::constants::ALL_TYPES
                    .iter()
                    .filter(|v| matches!(v.as_ref(), $type))
                    .map(|v| v.type_of())
                    .collect::<Arc<[_]>>()
                    .join(" or ");

                return crate::interpreter::builtins::errors::new_type_error_with_got_and_pos(
                    $name,
                    i,
                    &type_name,
                    arg.type_of(),
                );
            }
        }
    };
}

pub(crate) use type_check;

fn typecheck_args<F>(
    name: &str,
    typename: &str,
    condition: F,
    args: &[Reference],
) -> Option<Reference>
where
    F: Fn(&Reference) -> bool,
{
    for arg in args.iter() {
        if condition(arg) {
            return Some(new_type_error_with_got(name, typename, arg.type_of()));
        }
    }
    None
}

/// Adds all builtin functions to the environment
pub fn add_generic_builtins(env: &mut Env) {
    add_number_builtins(env);
    add_list_builtins(env);
    add_string_builtins(env);

    let functions: [(&str, BuiltinFunction); _] = [
        ("type", TYPE_OF),
        ("len", LEN),
        ("==", EQUALS),
        ("!=", NOT_EQUALS),
        ("<", LESSER_THAN),
        (">", GREATHER_THAN),
    ];

    functions
        .into_iter()
        .map(|(name, function)| (name, Reference::new(Object::Builtin { function })))
        .for_each(|(name, function)| {
            env.insert(name.into(), function.clone());
            env.insert(("std/".to_owned() + name).into(), function);
        });
}

/// Returns the length of a list or string
const LEN: BuiltinFunction = |_: &mut Program, args: Vec<Reference>| -> Reference {
    type_check!("len", args, [Object::String(_) | Object::List(_)]);

    match args[0].as_ref() {
        Object::String(s) => Reference::new(Object::Integer(s.len() as isize)),
        Object::List(l) => Reference::new(Object::Integer(l.len() as isize)),
        _ => unreachable!(),
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

const TYPE_OF: BuiltinFunction = |_, args| {
    args.first().map_or_else(
        || NULL.clone(),
        |v| Reference::new(Object::String(v.type_of().into())),
    )
};
