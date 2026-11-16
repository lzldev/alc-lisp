#![feature(macro_metavar_expr)]
#![feature(test)]

pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod utils;

#[cfg(feature = "bin")]
pub mod repl;

#[cfg(feature = "bin")]
pub mod native;

#[cfg(test)]
mod bench;

#[cfg(test)]
mod test;
