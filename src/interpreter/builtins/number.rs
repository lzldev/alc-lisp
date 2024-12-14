use std::rc::Rc;

use crate::interpreter::{objects::Object, Env};

use super::typecheck_args;

pub fn add_number_builtins(env: &mut Env) {
    env.insert(
        "+".into(),
        Rc::new(Object::Builtin {
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

                return Rc::new(Object::Integer(sum));
            },
        }),
    );

    env.insert(
        "-".into(),
        Rc::new(Object::Builtin {
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

                return Rc::new(Object::Integer(total));
            },
        }),
    );

    env.insert(
        "*".into(),
        Rc::new(Object::Builtin {
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

                return Rc::new(Object::Integer(total));
            },
        }),
    );

    env.insert(
        "/".into(),
        Rc::new(Object::Builtin {
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
                        return Rc::new(Object::Error(format!("division by zero")));
                    }

                    total /= n;
                }

                return Rc::new(Object::Integer(total));
            },
        }),
    );
}
