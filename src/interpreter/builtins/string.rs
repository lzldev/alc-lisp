use std::rc::Rc;

use crate::interpreter::{objects::Object, Env, STRING};

use super::typecheck_args;

pub fn add_string_builtins(env: &mut Env) {
    env.insert(
        "str".into(),
        Rc::new(Object::Builtin {
            function: |args| {
                if let Some(err) = typecheck_args(
                    "str",
                    STRING.type_of(),
                    |obj| !matches!(obj.as_ref(), Object::String(_)),
                    &args,
                ) {
                    return err;
                }

                let result = args
                    .iter()
                    .map(|v| {
                        let Object::String(inner) = v.as_ref() else {
                            panic!("This should never happen");
                        };

                        inner
                    })
                    .cloned()
                    .collect::<String>();

                Rc::new(Object::String(result))
            },
        }),
    );
}
