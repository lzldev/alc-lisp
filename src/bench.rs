extern crate test;

use crate::test::{new_test_program, prepare_code, prepare_test_ast, prepare_test_lexer};

use test::Bencher;

use dir_bench::dir_bench;

use paste::paste;

macro_rules! alc_f_bench {
    ($name:ident, $input:expr) => {
        paste! { #[bench]
            fn [<bench_ $name>](b: &mut Bencher) {
                let mut program = new_test_program();
                let ast = prepare_code(include_str!($input).to_string()).unwrap();

                b.iter(|| program.eval(&ast).unwrap());
            }

            #[bench]
            fn [<bench_ $name _cloned>](b: &mut Bencher) {
                let program = new_test_program();
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

#[dir_bench(
    dir: "$CARGO_MANIFEST_DIR/examples/benchs/lexer",
    glob: "**/*.alc",
)]
fn lexer(b: &mut Bencher, file: dir_bench::Fixture<&str>) {
    let _path = file.path();
    let code = file.into_content();

    let lexer = prepare_test_lexer(code.to_owned()).unwrap();

    b.iter(|| {
        let mut bench_lexer = lexer.clone();
        bench_lexer.parse().unwrap();

        println!("{:?}", bench_lexer.tokens());
    })
}

#[dir_bench(
    dir: "$CARGO_MANIFEST_DIR/examples/benchs/ast",
    glob: "**/*.alc",
)]
fn ast(b: &mut Bencher, file: dir_bench::Fixture<&str>) {
    let _path = file.path();
    let code = file.into_content();

    let mut lexer = prepare_test_lexer(code.to_owned()).unwrap();
    lexer.parse().unwrap();

    let ast = prepare_test_ast(lexer.tokens()).unwrap();

    b.iter(|| {
        let mut new_ast = ast.clone();

        let root = new_ast.parse().unwrap();

        if new_ast.has_errors() {
            panic!("error in AST: {:?}", new_ast.errors());
        }

        println!("{:?}", root);
    })
}
