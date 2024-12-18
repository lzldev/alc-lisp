use std::cell::LazyCell;

use crate::interpreter::{
    builtins::{add_generic_builtins, native::add_native_builtins},
    Env,
};

pub const NATIVE_ENV: LazyCell<Env> = LazyCell::new(|| {
    let mut globals: Env = Env::default();

    add_generic_builtins(&mut globals);
    add_native_builtins(&mut globals);

    globals
});
