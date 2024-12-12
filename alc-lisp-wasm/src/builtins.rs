use std::rc::Rc;

use alc_lisp::interpreter::{objects::Object, Env, NULL};
use log::info;

pub fn add_wasm_builtins(env: &mut Env) {
    env.insert(
        "print".into(),
        Rc::new(Object::Builtin(|args| {
            info!(
                "{}",
                args.iter().map(|v| format!("{}", v)).collect::<String>()
            );

            return NULL.clone();
        })),
    );

    env.insert(
        "debug".into(),
        Rc::new(Object::Builtin(|args| {
            info!(
                "{:?}",
                args.iter().map(|v| format!("{}", v)).collect::<String>()
            );
            return NULL.clone();
        })),
    );
}
