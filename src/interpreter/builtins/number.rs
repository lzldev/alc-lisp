//! Builtin functions for arithmetic operations
use crate::interpreter::{
    builtins::type_check,
    objects::{BuiltinFunction, Object},
    Env, Program, Reference, NUMBER, STRING,
};

use super::{
    errors::{new_args_len_error, new_type_error_with_pos},
    typecheck_args,
};

/// Add arithmetic builtins to the environment
pub fn add_number_builtins(env: &mut Env) {
    let functions: [(&str, BuiltinFunction); _] = [
        ("+", add),
        ("-", subtract),
        ("*", multiply),
        ("/", divide),
        ("%", MOD),
        ("parse_int", parse_int),
        ("abs", ABS),
    ];

    functions
        .into_iter()
        .map(|(name, function)| (name, Reference::new(Object::Builtin { function })))
        .for_each(|(name, function)| {
            env.insert(name.into(), function.clone());
            env.insert(("std/".to_owned() + name).into(), function);
        });
}

/// Adds numbers
pub fn add(_: &mut Program, args: Vec<Reference>) -> Reference {
    if let Some(err) = typecheck_args(
        "+",
        NUMBER.type_of(),
        |obj| !matches!(obj.as_ref(), Object::Integer(_)),
        &args,
    ) {
        return err;
    };

    let mut sum = 0;

    for obj in args.iter() {
        let Object::Integer(n) = obj.as_ref() else {
            panic!("This should never happen")
        };

        sum += n;
    }

    Reference::new(Object::Integer(sum))
}

/// Subtracts numbers
pub fn subtract(_: &mut Program, args: Vec<Reference>) -> Reference {
    if let Some(err) = typecheck_args(
        "-",
        NUMBER.type_of(),
        |obj| !matches!(obj.as_ref(), Object::Integer(_)),
        &args,
    ) {
        return err;
    };

    let Object::Integer(mut total) = args[0].as_ref() else {
        panic!("This should never happen");
    };

    for obj in args.iter().skip(1) {
        let Object::Integer(n) = obj.as_ref() else {
            panic!("This should never happen")
        };

        total -= n;
    }

    Reference::new(Object::Integer(total))
}

/// Multiplies numbers
pub fn multiply(_: &mut Program, args: Vec<Reference>) -> Reference {
    if let Some(err) = typecheck_args(
        "*",
        NUMBER.type_of(),
        |obj| !matches!(obj.as_ref(), Object::Integer(_)),
        &args,
    ) {
        return err;
    }

    let Object::Integer(mut total) = args[0].as_ref() else {
        panic!("This should never happen");
    };

    for obj in args.iter().skip(1) {
        let Object::Integer(n) = obj.as_ref() else {
            panic!("This should never happen")
        };

        total *= n;
    }

    Reference::new(Object::Integer(total))
}

/// Divides numbers
pub fn divide(_: &mut Program, args: Vec<Reference>) -> Reference {
    if let Some(err) = typecheck_args(
        "/",
        NUMBER.type_of(),
        |obj| !matches!(obj.as_ref(), Object::Integer(_)),
        &args,
    ) {
        return err;
    }

    let Object::Integer(mut total) = args[0].as_ref() else {
        panic!("This should never happen");
    };

    for obj in args.iter().skip(1) {
        let Object::Integer(n) = obj.as_ref() else {
            panic!("This should never happen")
        };

        if n == &0 {
            return Reference::new(Object::Error("division by zero".into()));
        }

        total /= n;
    }

    Reference::new(Object::Integer(total))
}

pub fn parse_int(_: &mut Program, args: Vec<Reference>) -> Reference {
    let len = args.len();
    if len != 1 {
        return new_args_len_error("sort", &args, 1);
    }

    let Object::String(input) = args[0].as_ref() else {
        return new_type_error_with_pos("parse_int", STRING.type_of(), 0);
    };

    if let Ok(value) = input.parse::<isize>() {
        Reference::new(Object::Integer(value))
    } else {
        Reference::new(Object::Error("Could not parse int".into()))
    }
}

pub const MOD: BuiltinFunction = |_, args| {
    type_check!("mod", args, [Object::Integer(_), Object::Integer(_)]);

    let Object::Integer(l) = args[0].as_ref() else {
        panic!("This should never happen");
    };
    let Object::Integer(r) = args[1].as_ref() else {
        panic!("This should never happen");
    };

    Reference::new(Object::Integer(l % r))
};

pub const ABS: BuiltinFunction = |_, args| {
    type_check!("mod", args, [Object::Integer(_)]);

    let Object::Integer(l) = args[0].as_ref() else {
        panic!("This should never happen");
    };

    Reference::new(Object::Integer(l.abs()))
};
