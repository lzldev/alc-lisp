use crate::interpreter::{objects::Object, Env, Reference};

use super::typecheck_args;

pub fn add_number_builtins(env: &mut Env) {
    env.insert(
        "+".into(),
        Reference::new(Object::Builtin {
            function: |args| {
                if let Some(err) = typecheck_args(
                    "+",
                    "integer",
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
            },
        }),
    );

    env.insert(
        "-".into(),
        Reference::new(Object::Builtin {
            function: |args| {
                if let Some(err) = typecheck_args(
                    "-",
                    "integer",
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
            },
        }),
    );

    env.insert(
        "*".into(),
        Reference::new(Object::Builtin {
            function: |args| {
                if let Some(err) = typecheck_args(
                    "*",
                    "integer",
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
            },
        }),
    );

    env.insert(
        "/".into(),
        Reference::new(Object::Builtin {
            function: |args| {
                if let Some(err) = typecheck_args(
                    "/",
                    "integer",
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
            },
        }),
    );
}
