use anyhow::Result;
use colored::Colorize;
use dir_test::{dir_test, Fixture};

use crate::{
    ast::{Node, AST},
    interpreter::{objects::Object, Env, Program},
    lexer::{Lexer, Token},
    native::NATIVE_ENV,
};

pub(crate) fn new_test_program() -> Program {
    let globals: Env = NATIVE_ENV.clone();

    Program::new(globals)
}

pub(crate) fn prepare_test_lexer(input: String) -> Result<Lexer> {
    Ok(Lexer::from_string(input))
}

pub(crate) fn prepare_test_ast(tokens: Vec<Token>) -> Result<AST> {
    Ok(AST::with_tokens(tokens))
}

pub(crate) fn prepare_code(input: String) -> Result<Node> {
    let mut lexer = prepare_test_lexer(input)?;
    lexer.parse()?;

    let tokens = lexer.tokens();

    let mut ast = prepare_test_ast(tokens)?;

    let root = ast.parse()?;

    if ast.has_errors() {
        ast.print_errors(&root);
        panic!("Error while parsing code");
    }

    Ok(root)
}

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/examples/tests",
    glob: "**/*.alc",
)]
fn example_benchs(fixture: Fixture<&str>) {
    // The file content and the absolute path of the file are available as follows.
    let path = fixture.path();
    let code = fixture.content();

    println!("{}:", path.purple());
    println!("{}", code);

    let mut program = new_test_program();
    let ast = prepare_code((*code).to_owned()).unwrap();

    let _last = program.eval(&ast).expect("running code failed");

    let callstack = program.get_env();
    let envs = callstack.active_slice();
    let global = envs[0].read().clone();

    let expected = global.get("expected").expect("expected value not found");
    let message = global.get("message").expect("message value not found");
    let Object::String(message) = message.as_ref() else {
        panic!("message value is not a string");
    };
    let output = global.get("output").expect("output value not found");

    assert_eq!(expected, output, "{message}");
}

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/examples/benchs/lexer",
    glob: "**/*.alc",
)]
fn example_tests(fixture: Fixture<&str>) {
    // The file content and the absolute path of the file are available as follows.
    let path = fixture.path();
    let code = fixture.into_content();

    println!("{}", path.purple());
    println!("{}", code);

    let mut program = new_test_program();
    let ast = prepare_code((*code).to_owned()).unwrap();

    let _last = program.eval(&ast).expect("running code failed");

    let callstack = program.get_env();
    let envs = callstack.active_slice();
    let global = envs[0].read().clone();

    let expected = global.get("expected").expect("expected value not found");
    let message = global.get("message").expect("message value not found");
    let Object::String(message) = message.as_ref() else {
        panic!("message value is not a string");
    };
    let output = global.get("output").expect("output value not found");

    assert_eq!(expected, output, "{message}");
}
