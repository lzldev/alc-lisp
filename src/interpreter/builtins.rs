use std::rc::Rc;

use super::{objects::Object, Env};

pub fn add_builtins(env: &mut Env) {
    env.insert(
        "+".into(),
        Rc::new(Object::Builtin(|args| {
            if args.iter().any(|obj| !matches!(obj, Object::Integer(_))) {
                return Object::Error(format!(
                    "Invalid argument type for function '+': args should be numbers",
                ));
            };

            let mut sum = 0;

            for obj in args.iter() {
                let Object::Integer(n) = obj else {
                    panic!("This should never happen")
                };

                sum += n;
            }

            return Object::Integer(sum);
        })),
    );

    env.insert(
        "-".into(),
        Rc::new(Object::Builtin(|args| {
            if args.iter().any(|obj| !matches!(obj, Object::Integer(_))) {
                return Object::Error(format!(
                    "Invalid argument type for function '-': args should be numbers",
                ));
            };

            let Object::Integer(mut total) = args[0] else {
                panic!("This should never happen");
            };

            for obj in args.iter().skip(1) {
                let Object::Integer(n) = obj else {
                    panic!("This should never happen")
                };

                total -= n;
            }

            return Object::Integer(total);
        })),
    );

    env.insert(
        "*".into(),
        Rc::new(Object::Builtin(|args| {
            if args.iter().any(|obj| !matches!(obj, Object::Integer(_))) {
                return Object::Error(format!(
                    "Invalid argument type for function '*': args should be numbers",
                ));
            };

            let Object::Integer(mut total) = args[0] else {
                panic!("This should never happen");
            };

            for obj in args.iter().skip(1) {
                let Object::Integer(n) = obj else {
                    panic!("This should never happen")
                };

                total *= n;
            }

            return Object::Integer(total);
        })),
    );

    env.insert(
        "/".into(),
        Rc::new(Object::Builtin(|args| {
            if args.iter().any(|obj| !matches!(obj, Object::Integer(_))) {
                return Object::Error(format!(
                    "Invalid argument type for function '/': args should be numbers",
                ));
            };

            let Object::Integer(mut total) = args[0] else {
                panic!("This should never happen");
            };

            for obj in args.iter().skip(1) {
                let Object::Integer(n) = obj else {
                    panic!("This should never happen")
                };

                if n == &0 {
                    return Object::Error(format!("division by zero"));
                }

                total /= n;
            }

            return Object::Integer(total);
        })),
    );
}
