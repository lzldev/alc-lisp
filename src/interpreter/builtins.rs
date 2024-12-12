use std::rc::Rc;

use crate::interpreter::NULL;

use super::{bool_from_native, objects::Object, Env, Reference, TRUE};

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
        return Some(Rc::new(Object::Error(format!(
            "Invalid argument type for function '{}': args should be {}",
            name, typename
        ))));
    };
    None
}

pub fn add_generic_builtins(env: &mut Env) {
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

    env.insert(
        "==".into(),
        Rc::new(Object::Builtin {
            function: |args| {
                let len = args.len();
                if len < 1 || len == 0 {
                    return Rc::new(Object::Error(format!(
                        "Invalid argument type for function '==': got: {} expected: 2",
                        args.len()
                    )));
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
        Rc::new(Object::Builtin {
            function: |args| {
                let len = args.len();
                if len < 1 || len == 0 {
                    return Rc::new(Object::Error(format!(
                        "Invalid argument type for function '<': got: {}",
                        args.len()
                    )));
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
        Rc::new(Object::Builtin {
            function: |args| {
                let len = args.len();
                if len < 1 || len == 0 {
                    return Rc::new(Object::Error(format!(
                        "Invalid argument type for function '<': got: {}",
                        args.len()
                    )));
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

    env.insert(
        "str".into(),
        Rc::new(Object::Builtin {
            function: |args| {
                if let Some(err) = typecheck_args(
                    "str",
                    "string",
                    |obj| !matches!(obj.as_ref(), Object::String(_)),
                    &args,
                ) {
                    return err;
                }

                let mut result = {
                    let Object::String(inner) = args[0].as_ref() else {
                        panic!("This should never happen");
                    };
                    inner.clone()
                };

                for obj in args.iter().skip(1) {
                    let Object::String(s) = obj.as_ref() else {
                        panic!("This should never happen")
                    };

                    result.push_str(s.as_str());
                }

                Reference::new(Object::String(result))
            },
        }),
    );
}

pub fn add_native_builtins(env: &mut Env) {
    env.insert(
        "print".into(),
        Rc::new(Object::Builtin {
            function: |args| {
                println!(
                    "{}",
                    args.iter().map(|v| format!("{}", v)).collect::<String>()
                );
                return NULL.clone();
            },
        }),
    );

    env.insert(
        "debug".into(),
        Rc::new(Object::Builtin {
            function: |args| {
                println!("{:?}", args);
                return NULL.clone();
            },
        }),
    );
}
