extern crate test;

use test::Bencher;

use crate::{
    ast::{Node, AST},
    interpreter::{Env, Program},
    lexer::Lexer,
    native::NATIVE_ENV,
};

use paste::paste;

macro_rules! alc_f_bench {
    ($name:ident, $input:expr) => {
        paste! {
            #[bench]
            fn [<bench_ $name>](b: &mut Bencher) {
                let mut program = new_native_program();
                let ast = prepare_code(include_str!($input).to_string()).unwrap();

                b.iter(|| program.eval(&ast).unwrap());
            }

            #[bench]
            fn [<bench_ $name _cloned>](b: &mut Bencher) {
                let program = new_native_program();
                let ast = prepare_code(include_str!($input).to_string()).unwrap();

                b.iter(|| program.clone().eval(&ast).unwrap());
            }
        }
    };
}

alc_f_bench!(hello_world, "../examples/hello_world.alc");
alc_f_bench!(hello_world_concat, "../examples/hello_string_concat.alc");
alc_f_bench!(test_test, "../examples/test.alc");
alc_f_bench!(fib_nth, "../examples/fib.alc");
alc_f_bench!(fib_list_flat, "../examples/fib_list_flat.alc");
alc_f_bench!(fib_list_concat, "../examples/fib_list_concat.alc");
alc_f_bench!(map_test, "../examples/map_test.alc");
alc_f_bench!(concat_test, "../examples/concat_test.alc");
alc_f_bench!(split_test, "../examples/split_test.alc");
alc_f_bench!(reduce_test, "../examples/reduce_test.alc");
alc_f_bench!(reduce_sum, "../examples/reduce_sum_test.alc");

fn new_native_program() -> Program {
    let globals: Env = NATIVE_ENV.clone();

    Program::new(globals)
}

fn prepare_code(input: String) -> anyhow::Result<Node> {
    let mut lexer = Lexer::from_string(input);
    lexer.parse()?;

    let tokens = lexer.tokens();

    let mut ast = AST::with_tokens(tokens);

    let root = ast.parse()?;

    if ast.has_errors() {
        ast.print_errors(&root);
        panic!("Error while parsing code");
    }

    Ok(root)
}
