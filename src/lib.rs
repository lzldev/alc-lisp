#![feature(allocator_api)]
#![feature(test)]
#![feature(lock_value_accessors)]

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
