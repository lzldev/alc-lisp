use std::{collections::HashSet,  sync::Mutex};

use alc_lisp::interpreter::{objects::Object, Env, Reference, NULL};
use js_sys::{Array, Function};
use log::info;
use once_cell::sync::Lazy;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::function_container::FunctionContainer;

#[wasm_bindgen(typescript_custom_section)]
const ADD_PRINT_CALLBACK_TYPE: &'static str = r#"
type PrintCallbackFn = (...objs:Object[]) => void;
type AddPrintCallBackFn = (callback:PrintCallbackFn) => void;
"#;

static CALLBACKS: Lazy<Mutex<HashSet<FunctionContainer>>> =
    Lazy::new(|| Mutex::new(HashSet::new()));

#[wasm_bindgen]
pub fn print_callbacks() {
    let callbacks = CALLBACKS.lock().unwrap();

    info!("l:{}\n{:?}", callbacks.len(), callbacks);
}

#[wasm_bindgen]
pub fn add_print_callback(callback: Function) {
    let mut callbacks = CALLBACKS.lock().unwrap();

    callbacks.insert(FunctionContainer { function: callback });
}

#[wasm_bindgen]
pub fn remove_print_callback(callback: Function) {
    let mut callbacks = CALLBACKS.lock().unwrap();

    let contained = &FunctionContainer { function: callback };

    callbacks
        .contains(contained)
        .then(|| callbacks.remove(contained));
}

pub fn add_wasm_builtins(env: &mut Env) {
    env.insert(
        "print".into(),
        Reference::new(Object::Builtin {
            function: |args| {
                info!(
                    "{}",
                    args.iter().map(|v| format!("{}", v)).collect::<String>()
                );

                let callbacks = CALLBACKS.lock().unwrap();

                if callbacks.is_empty() {
                    return NULL.clone();
                }

                let array = Array::new_with_length(args.len() as u32);

                for (idx, arg) in args.iter().enumerate() {
                    array.set(idx as u32, JsValue::from(arg.as_ref().clone()));
                }

                callbacks.iter().for_each(|function| {
                    function
                        .function
                        .apply(&JsValue::NULL, &array)
                        .expect("to call print callback");
                });

                return NULL.clone();
            },
        }),
    );

    env.insert(
        "debug".into(),
        Reference::new(Object::Builtin {
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
