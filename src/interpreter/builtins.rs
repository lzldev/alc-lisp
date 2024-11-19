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
                match obj {
                    Object::Integer(n) => sum += n,
                    _ => panic!("This should never happen"),
                };
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

            let Object::Integer(mut sum) = args[0] else {
                panic!("This should never happen");
            };

            for obj in args.iter().skip(1) {
                match obj {
                    Object::Integer(n) => sum -= n,
                    _ => panic!("This should never happen"),
                };
            }

            return Object::Integer(sum);
        })),
    );
}

fn arg_len_error(size: usize, expected: usize) -> Object {
    return Object::Error(format!("Invalid args len {} expected:{}", size, expected));
}
