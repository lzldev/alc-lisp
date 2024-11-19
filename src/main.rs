use std::{collections::HashMap, rc::Rc, time};

use alc_lisp::{
    ast::{Node, AST},
    interpreter::{
        builtins::add_builtins,
        objects::{self, Object},
        Env, Program,
    },
    lexer::Lexer,
};

fn main() {
    let test_file = std::fs::read_to_string("./test_4.txt").expect("to open file");
    let mut lexer = Lexer::from_string(test_file);

    let _t = Timer::new("Total");
    {
        let _t = Timer::new("Lexer");
        lexer.parse().expect("lexer::parse");
    }

    let tokens = lexer.tokens();
    println!("LEXER\n----{}\n----", lexer.to_string());
    // dbg!(&tokens);

    let mut ast = AST::with_tokens(tokens);

    let root: Node;
    {
        let _t = Timer::new("AST");
        root = ast.parse().expect("ast::parse");

        // dbg!(&root);
        if ast.has_errors() {
            ast.print_errors(&root);
        }
    }

    let mut globals: Env = HashMap::new();

    add_builtins(&mut globals);

    let mut program = Program::new(globals);

    let result: Object;
    {
        let _t = Timer::new("Interpreter");
        result = program.eval(&root).expect("error running program:");
        dbg!(result);
    }
}
struct Timer {
    name: String,
    start: time::Instant,
}

impl Timer {
    fn new(name: &str) -> Self {
        Timer {
            name: name.to_string(),
            start: time::Instant::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let took = time::Instant::now().duration_since(self.start);

        println!("{}: {:?}", self.name, took);
    }
}
