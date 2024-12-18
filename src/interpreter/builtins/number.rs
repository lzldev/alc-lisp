//! Builtin functions for arithmetic operations
use crate::interpreter::{objects::Object, Env, Reference, NUMBER};

use super::typecheck_args;

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
