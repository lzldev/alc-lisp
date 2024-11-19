use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

use alc_lisp::{
    ast::{Node, AST},
    interpreter::{builtins::add_builtins, Env, Program},
    lexer::Lexer,
};

fn main() -> anyhow::Result<()> {
    let mut globals: Env = HashMap::new();
    add_builtins(&mut globals);

    let mut globals = Option::Some(globals);

    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        print!(">> ");
        stdout.flush()?;
        let mut line = String::new();
        let read = stdin.read_line(&mut line)?;

        if read == 0 || line == ".q\n" {
            break;
        }

        let mut lexer = Lexer::from_string(line);
        lexer.parse()?;

        let tokens = lexer.tokens();
        dbg!(&tokens);

        let mut ast = AST::with_tokens(tokens);

        let root: Node;
        {
            root = ast.parse().expect("ast::parse");

            dbg!(&root);
            if ast.has_errors() {
                ast.print_errors(&root);
            }
        }

        let taken = globals.take().unwrap();
        let mut program = Program::new(taken);

        let result = program.eval(&root)?;

        println!("{:?}", result);

        globals = Some(program.env[0].take())
    }

    // let result: Object;
    // {
    //     result = program.eval(&root).expect("error running program:");
    //     dbg!(result);
    // }

    Ok(())
}
