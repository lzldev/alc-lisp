use std::cell::LazyCell;

use js_sys::JsString;
use wasm_bindgen::JsValue;

use super::Object;

pub const BUILTIN_MESSAGE: LazyCell<wasm_bindgen::JsValue> =
    LazyCell::new(|| JsString::from("Builtin Function").into());

impl From<Object> for wasm_bindgen::JsValue {
    fn from(value: Object) -> Self {
        match value {
            Object::Null => wasm_bindgen::JsValue::NULL,
            Object::Integer(value) => wasm_bindgen::JsValue::from(value as i32),
            Object::String(st) => JsString::from(st).into(),
            Object::Bool(value) => {
                if value {
                    wasm_bindgen::JsValue::TRUE
                } else {
                    wasm_bindgen::JsValue::FALSE
                }
            }
            Object::List(vec) => {
                let array = js_sys::Array::new_with_length(vec.len() as u32);

                vec.into_iter().enumerate().for_each(|(idx, arg)| {
                    array.set(idx as u32, JsValue::from((*arg).clone()));
                });

                array.into()
            }
            Object::Builtin { .. } => BUILTIN_MESSAGE.clone(),
            Object::Function { env, .. } => JsString::from(format!("FUNCTION [{:p}]", env)).into(),
            Object::Error(_) => todo!(),
        }
    }
}
