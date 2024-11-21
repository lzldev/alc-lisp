use alc_lisp::repl::{start_repl, ReplArgs};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = ReplArgs::parse();

    start_repl(&args)
}
