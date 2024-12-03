use std::{cell::LazyCell, collections::HashMap, panic, rc::Rc};

use alc_lisp::{
    ast::AST,
    interpreter::{builtins::add_generic_builtins, objects::Object, Env, Program, NULL},
    lexer::Lexer,
};
use log::info;
use wasm_bindgen::prelude::*;
use web_sys::{Performance, Window};

#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[wasm_bindgen]
pub fn add2(left: usize, right: usize) -> usize {
    left + right
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
