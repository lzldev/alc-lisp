use std::rc::Rc;

use crate::interpreter::{objects::Object, Env, NULL};

use super::errors::{new_args_len_error, new_type_error_with_pos};

pub fn add_list_builtins(env: &mut Env) {
    env.insert(
        "nth".into(),
        Rc::new(Object::Builtin {
            function: |args| {
                let len = args.len();
                if len != 2 {
                    return new_args_len_error("nth", &args, 2);
                }

                let Object::Integer(n) = args[0].as_ref() else {
                    return new_type_error_with_pos("nth", "number", 1);
                };
                let Object::List(l) = args[1].as_ref() else {
                    return new_type_error_with_pos("nth", "list", 2);
                };

                return l
                    .get(*n as usize)
                    .map(|v| v.clone())
                    .unwrap_or(NULL.clone());
            },
        }),
    );

    env.insert(
        "head".into(),
        Rc::new(Object::Builtin {
            function: |args| {
                let len = args.len();
                if len != 1 {
                    return new_args_len_error("head", &args, 1);
                }

                let Object::List(l) = args[0].as_ref() else {
                    return new_type_error_with_pos("head", "list", 1);
                };

                if l.len() == 0 {
                    return NULL.clone();
                }

                let vec = l.iter().take(1).cloned().collect();

                return Rc::new(Object::List(vec));
            },
        }),
    );

    env.insert(
        "tail".into(),
        Rc::new(Object::Builtin {
            function: |args| {
                let len = args.len();
                if len != 1 {
                    return new_args_len_error("tail", &args, 1);
                }

                let Object::List(l) = args[0].as_ref() else {
                    return new_type_error_with_pos("tail", "list", 1);
                };

                let vec = l.iter().skip(1).cloned().collect();

                return Rc::new(Object::List(vec));
            },
        }),
    );

    env.insert(
        "slice".into(),
        Rc::new(Object::Builtin {
            function: |args| {
                let len = args.len();
                if len != 2 && len != 3 {
                    return new_args_len_error("slice", &args, 2);
                }

                let Object::List(l) = args[0].as_ref() else {
                    return new_type_error_with_pos("slice", "list", 0);
                };
                let Object::Integer(n) = args[1].as_ref() else {
                    return new_type_error_with_pos("slice", "integer", 1);
                };

                let end = if let Some(Object::Integer(end)) = args.get(2).map(|v| v.as_ref()) {
                    *end as usize
                } else {
                    l.len()
                };

                let vec = l
                    .iter()
                    .skip(*n as usize)
                    .take(end - *n as usize)
                    .cloned()
                    .collect();

                return Rc::new(Object::List(vec));
            },
        }),
    );
}
