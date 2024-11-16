use std::time;

use alc_lisp::{
    ast::{Program, AST},
    lexer::Lexer,
};

fn main() {
    let test_file = std::fs::read_to_string("./test_2.txt").expect("to open file");
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

    let program: Program;
    {
        let _t = Timer::new("AST");
        program = ast.parse().expect("ast::parse");

        dbg!(&program);
        if ast.has_errors() {
            ast.print_errors(&program.root);
        }
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
