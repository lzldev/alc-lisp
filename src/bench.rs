#![allow(dead_code)]

extern crate test;

use test::Bencher;

use crate::{
    ast::{Node, AST},
    interpreter::{Env, Program},
    lexer::Lexer,
    native::NATIVE_ENV,
};

#[bench]
fn bench_nth_fib(b: &mut Bencher) {
    let program = new_native_program();
    let ast = prepare_code(include_str!("../examples/fib.alc").to_string()).unwrap();

    b.iter(|| {
        program.clone().eval(&ast).unwrap();
    });
}

fn new_native_program() -> Program {
    let globals: Env = NATIVE_ENV.clone();

    Program::new(globals)
}

fn prepare_code(input: String) -> anyhow::Result<Node> {
    let lexer = Lexer::from_string(input);

    let tokens = lexer.tokens();

    let mut ast = AST::with_tokens(tokens);

    let root = ast.parse()?;

    if ast.has_errors() {
        ast.print_errors(&root);
    }

    Ok(root)
}

fn run_string(input: String) -> anyhow::Result<()> {
    let lexer = Lexer::from_string(input);

    let tokens = lexer.tokens();

    let mut ast = AST::with_tokens(tokens);

    let root = ast.parse()?;

    if ast.has_errors() {
        ast.print_errors(&root);
    }

    let globals: Env = NATIVE_ENV.clone();
    let mut program = Program::new(globals);
    program.eval(&root)?;

    Ok(())
}
