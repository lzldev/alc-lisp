use std::{cell::LazyCell, collections::HashMap};

use crate::interpreter::{
    builtins::{add_generic_builtins, native::add_native_builtins},
    Env,
};

pub const NATIVE_ENV: LazyCell<Env> = LazyCell::new(|| {
    let mut globals: Env = HashMap::new();

    add_generic_builtins(&mut globals);
    add_native_builtins(&mut globals);

    globals
});
