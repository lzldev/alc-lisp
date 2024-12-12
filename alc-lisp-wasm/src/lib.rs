use std::{cell::LazyCell, collections::HashMap, panic, rc::Rc};

use alc_lisp::{
    ast::AST,
    interpreter::{builtins::add_generic_builtins, objects::Object, Env, Program, NULL},
    lexer::Lexer,
};

use log::info;
use wasm_bindgen::prelude::*;
use web_sys::{Performance, Window};

#[wasm_bindgen(typescript_custom_section)]
const TYPES_EXTENSION: &str = include_str!("../target/types.ts"); //Generated from `build.rs`

#[wasm_bindgen(start)]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().expect("console_log::init");
}

pub const WINDOW: LazyCell<Window> = LazyCell::new(|| web_sys::window().expect("window not found"));
pub const PERFORMANCE: LazyCell<Performance> =
    LazyCell::new(|| WINDOW.performance().expect("performance not found"));

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

#[wasm_bindgen]
pub fn get_ast(code: String, callback: js_sys::Function) {
    let mut lexer = Lexer::from_string(code);

    lexer.parse().expect("lexer::parse");

    let tokens = lexer.tokens();

    let mut ast = AST::with_tokens(tokens);

    let root = ast.parse().expect("ast::parse");

    if ast.has_errors() {
        ast.print_errors(&root);
        panic!("ast::has_errors");
    }

    let node = serde_wasm_bindgen::to_value(&root).expect("serde_wasm_bindgen::from_value");

    callback
        .call1(&JsValue::NULL, &node)
        .expect("error running callback");
}

use gloo_utils::format::JsValueSerdeExt;

#[wasm_bindgen]
pub fn get_ast_gloo(code: String, callback: js_sys::Function) {
    let mut lexer = Lexer::from_string(code);

    lexer.parse().expect("lexer::parse");

    let tokens = lexer.tokens();

    let mut ast = AST::with_tokens(tokens);

    let root = ast.parse().expect("ast::parse");

    if ast.has_errors() {
        ast.print_errors(&root);
        panic!("ast::has_errors");
    }

    let node = JsValue::from_serde(&root).expect("gloo_utils::format::JsValueSerdeExt::from_serde");

    callback
        .call1(&JsValue::NULL, &node)
        .expect("error running callback");
}

#[wasm_bindgen]
pub fn run(code: String) {
    let mut lexer = Lexer::from_string(code);

    lexer.parse().expect("lexer::parse");

    let tokens = lexer.tokens();

    let mut ast = AST::with_tokens(tokens);

    let root = ast.parse().expect("ast::parse");

    if ast.has_errors() {
        ast.print_errors(&root);
        panic!("ast::has_errors");
    }

    let mut globals: Env = HashMap::new();
    add_generic_builtins(&mut globals);
    add_wasm_builtins(&mut globals);

    let mut program = Program::new(globals);

    let start = PERFORMANCE.now(); // TODO:Remove Timing code
    let result = program.eval(&root).expect("program::eval");
    let end = PERFORMANCE.now() - start; // TODO:Remove Timing code

    info!("took:{:.4}ms", end);

    info!("result:{}", result);
}
