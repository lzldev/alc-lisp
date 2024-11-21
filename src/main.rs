use std::collections::HashMap;

use alc_lisp::{
    ast::{Node, AST},
    interpreter::{builtins::add_builtins, Env, Program},
    lexer::Lexer,
    repl::{start_repl, ReplArgs},
    utils::timer::Timer,
};
use anyhow::Context;
use clap::{arg, Parser, Subcommand};

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
    if args.debug {
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
        println!("LEXER\n----{}\n----", lexer.to_string());
        dbg!(&tokens);
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

    let mut globals: Env = HashMap::new();
    add_builtins(&mut globals);

    let mut program = Program::new(globals);

    let result = {
        let _t: Timer;
        if args.time {
            _t = Timer::new("Interpreter");
        }
        program.eval(&root)?
    };

    println!("{}", result);
    Ok(())
}
