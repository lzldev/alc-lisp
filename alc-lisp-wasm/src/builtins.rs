use std::{collections::HashSet, fmt::Write as _};

use alc_lisp::interpreter::{objects::Object, Env, Reference, NULL};
use js_sys::{Array, Function};
use log::info;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::function_container::FunctionContainer;

static CALLBACKS: Lazy<Mutex<HashSet<FunctionContainer>>> =
    Lazy::new(|| Mutex::new(HashSet::new()));

#[wasm_bindgen]
pub fn print_callbacks() {
    let callbacks = CALLBACKS.lock();

    info!("l:{}\n{:?}", callbacks.len(), callbacks);
}

#[wasm_bindgen]
pub fn add_print_callback(callback: Function) {
    let mut callbacks = CALLBACKS.lock();

    callbacks.insert(FunctionContainer { function: callback });
}

#[wasm_bindgen]
pub fn remove_print_callback(callback: Function) {
    let mut callbacks = CALLBACKS.lock();

    let contained = &FunctionContainer { function: callback };

    callbacks
        .contains(contained)
        .then(|| callbacks.remove(contained));
}

pub fn add_wasm_builtins(env: &mut Env) {
    env.insert(
        "print".into(),
        Reference::new(Object::Builtin {
            function: |_, args| {
                let callbacks = CALLBACKS.lock();

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

                NULL.clone()
            },
        }),
    );

    env.insert(
        "debug".into(),
        Reference::new(Object::Builtin {
            function: |_, args| {
                info!(
                    "{:?}",
                    args.iter().fold(String::new(), |mut output, b| {
                        let _ = write!(output, "{}", b);
                        output
                    })
                );
                NULL.clone()
            },
        }),
    );
}
