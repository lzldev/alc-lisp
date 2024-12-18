//! Builtin functions for arithmetic operations
use crate::interpreter::{objects::Object, Env, Reference, NUMBER, STRING};

use super::{
    errors::{new_args_len_error, new_type_error_with_pos},
    typecheck_args,
};

/// Add arithmetic builtins to the environment
pub fn add_number_builtins(env: &mut Env) {
    env.insert(
        "+".into(),
        Reference::new(Object::Builtin { function: add }),
    );

    env.insert(
        "-".into(),
        Reference::new(Object::Builtin { function: subtract }),
    );

    env.insert(
        "*".into(),
        Reference::new(Object::Builtin { function: multiply }),
    );

    env.insert(
        "/".into(),
        Reference::new(Object::Builtin { function: divide }),
    );

    env.insert(
        "parse_int".into(),
        Reference::new(Object::Builtin {
            function: parse_int,
        }),
    );
}

/// Adds numbers
pub fn add(args: Vec<Reference>) -> Reference {
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

    return Reference::new(Object::Integer(sum));
}

/// Subtracts numbers
pub fn subtract(args: Vec<Reference>) -> Reference {
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

    return Reference::new(Object::Integer(total));
}

/// Multiplies numbers
pub fn multiply(args: Vec<Reference>) -> Reference {
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

    return Reference::new(Object::Integer(total));
}

/// Divides numbers
pub fn divide(args: Vec<Reference>) -> Reference {
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
            return Reference::new(Object::Error(format!("division by zero")));
        }

        total /= n;
    }

    return Reference::new(Object::Integer(total));
}

pub fn parse_int(args: Vec<Reference>) -> Reference {
    let len = args.len();
    if len != 1 {
        return new_args_len_error("sort", &args, 1);
    }

    let Object::String(input) = args[0].as_ref() else {
        return new_type_error_with_pos("parse_int", &STRING.type_of(), 0);
    };

    if let Ok(value) = input.parse::<isize>() {
        return Reference::new(Object::Integer(value));
    } else {
        return Reference::new(Object::Error(format!("Could not parse int")));
    }
}
