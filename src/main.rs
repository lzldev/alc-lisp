use alc_lisp::{
    ast::{Node, AST},
    interpreter::{objects::Object, Env, Program},
    lexer::Lexer,
    native::NATIVE_ENV,
    repl::{start_repl, ReplArgs},
    utils::timer::Timer,
};
use anyhow::Context;
use clap::{arg, Parser, Subcommand};

use colored::Colorize;
#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    file_name: Option<String>,
    ///Time the execution of the program
    #[arg(short, long, default_value_t = false)]
    time: bool,

    ///Debug information
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    ///Show Lexer Debug information
    #[arg(long, default_value_t = false)]
    debug_lexer: bool,

    ///Show AST Debug information
    #[arg(long, default_value_t = false)]
    debug_ast: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ///Start the repl
    Repl(ReplArgs),
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match (&args.command, &args.file_name) {
        (Some(Commands::Repl(args)), _) => start_repl(args),
        (_, None) => start_repl(&ReplArgs::default()),
        _ => run_file(args),
    }
}

fn run_file(args: Args) -> anyhow::Result<()> {
    let file = std::fs::read_to_string(args.file_name.unwrap()).context("to open file:")?;

    let _t: Timer;
    if args.time {
        _t = Timer::new("Total");
    }

    let mut lexer = Lexer::from_string(file);

    {
        let _t: Timer;
        if args.time {
            _t = Timer::new("Lexer");
        }
        lexer.parse()?;
    }

    let tokens = lexer.tokens();

    if args.debug_lexer || args.debug {
        println!("LEXER\n----{}\n----", lexer);
    }

    let mut ast = AST::with_tokens(tokens);

    let root: Node = {
        let _t: Timer;
        if args.time {
            _t = Timer::new("AST");
        }

        ast.parse()?
    };

    if args.debug_ast || args.debug {
        dbg!(&root);
    }

    if ast.has_errors() {
        ast.print_errors(&root);
    }

    let globals: Env = NATIVE_ENV.clone();

    let mut program = Program::new(globals);

    let result = {
        let _t: Timer;
        if args.time {
            _t = Timer::new("Interpreter");
        }
        program.eval(&root)?
    };

    match result.as_ref() {
        Object::Error(err) => {
            println!("{}{}", "error:".red(), err);
        }
        v => {
            println!("{}", v);
        }
    }
    Ok(())
}
