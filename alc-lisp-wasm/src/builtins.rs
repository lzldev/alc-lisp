use std::rc::Rc;

use alc_lisp::interpreter::{objects::Object, Env, NULL};
use log::info;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const ADD_PRINT_CALLBACK_TYPE: &'static str = r#"
type AddPrintCallBackFn = (a:Object) => number;
"#;

pub fn add_wasm_builtins(env: &mut Env) {
    env.insert(
        "print".into(),
        Rc::new(Object::Builtin {
            function: |args| {
                info!(
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
                info!(
                    "{:?}",
                    args.iter().map(|v| format!("{}", v)).collect::<String>()
                );
                return NULL.clone();
            },
        }),
    );
}
