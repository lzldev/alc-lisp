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
        lexer.parse().unwrap();
    }
    let tokens = lexer.tokens();
    // dbg!(&tokens);

    let ast = AST::with_tokens(tokens);

    let program: Program;
    {
        let _t = Timer::new("AST");
        program = ast.parse().unwrap();
        dbg!(&program);
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
