use std::io::{stdin, stdout, Write};

use anyhow::Context;
use clap::{arg, Parser};
use colored::Colorize;

use crate::{interpreter::map_rust_error, native::NATIVE_ENV, utils::timer::Timer};

use super::{
    ast::{Node, AST},
    interpreter::Program,
    lexer::Lexer,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug, Default)]
pub struct ReplArgs {
    ///Time the execution
    #[arg(short, long, default_value_t = false)]
    time: bool,

    ///Show debug information
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    ///Show Lexer Debug information
    #[arg(long, default_value_t = false)]
    debug_lexer: bool,

    ///Show AST Debug information
    #[arg(long, default_value_t = false)]
    debug_ast: bool,
}

enum ReplResponse {
    Continue,
    Stop,
}

pub fn start_repl(repl_args: &ReplArgs) -> anyhow::Result<()> {
    println!("ALC_LISP [{}] REPL - INTERPRETER", VERSION);

    let mut globals = Option::Some(NATIVE_ENV.clone());

    let stdin = stdin();
    let mut stdout = stdout();

    let mut run_repl = || -> anyhow::Result<ReplResponse> {
        print!(">> ");
        stdout.flush()?;

        let mut line = String::new();
        let read = stdin.read_line(&mut line)?;

        let trimmed = line.trim_ascii();

        if read == 0 || trimmed.is_empty() {
            return Ok(ReplResponse::Continue);
        }

        if trimmed == ".q" {
            println!("ENDING {}", "REPL".bold());
            return Ok(ReplResponse::Stop);
        } else if trimmed == ".clear" {
            print!("{}[2J", 27 as char);
            stdout.flush()?;

            return Ok(ReplResponse::Continue);
        }

        let mut lexer = Lexer::from_string(line);
        lexer.parse().context("lexer::parse")?;

        let tokens = lexer.tokens();
        if repl_args.debug_lexer || repl_args.debug {
            dbg!(&tokens);
        }

        let mut ast = AST::with_tokens(tokens);

        let root: Node;
        {
            root = ast.parse().context("ast::parse")?;

            if repl_args.debug_ast || repl_args.debug {
                dbg!(&root);
            }

            if ast.has_errors() {
                ast.print_errors(&root);
            }
        }

        let taken = globals.take().unwrap();
        let mut program = Program::new(taken);

        let result = {
            let _t: Timer;
            if repl_args.time {
                _t = Timer::new("EVAL:");
            }
            let result = program.eval(&root).context("program::eval");
            globals = Some(program.get_env().active_slice()[0].read().clone());

            result.and_then(map_rust_error!("eval error"))?
        };

        println!("{}", result);

        Ok(ReplResponse::Continue)
    };

    loop {
        match run_repl() {
            Ok(ReplResponse::Continue) => continue,
            Ok(ReplResponse::Stop) => return Ok(()),
            Err(err) => {
                println!("{} {:?}", "error:".red(), err)
            }
        }
    }
}
