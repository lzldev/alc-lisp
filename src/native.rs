use std::sync::LazyLock;

use crate::interpreter::{
    builtins::{add_generic_builtins, native::add_native_builtins},
    Env,
};

pub static NATIVE_ENV: LazyLock<Env> = LazyLock::new(|| {
    let mut globals: Env = Env::default();

    add_generic_builtins(&mut globals);
    add_native_builtins(&mut globals);

    globals
});
