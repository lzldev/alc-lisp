//! Builtin functions to be used in a native environment

use std::{
    env::current_dir,
    fs::File,
    io::{Read, Seek},
    thread,
    time::Duration,
};

use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::interpreter::{
    objects::{BuiltinFunction, Object},
    Env, Reference, NULL, NUMBER, STRING,
};

use super::{
    errors::{new_args_len_error, new_type_error_with_pos},
    typecheck_args,
};

/// Adds the native builtins to a environment
pub fn add_native_builtins(env: &mut Env) {
    let functions: [(&str, BuiltinFunction); _] = [
        ("print", PRINT),
        ("debug", DEBUG),
        ("pdebug", PDEBUG),
        ("pwd", PWD),
        ("open", OPEN),
        ("close", CLOSE),
        ("file", FILE),
        ("read_file", READ_FILE),
        ("sleep", SLEEP),
    ];

    functions
        .into_iter()
        .map(|(name, function)| (name, Reference::new(Object::Builtin { function })))
        .for_each(|(name, function)| {
            env.insert(name.into(), function.clone());
            env.insert(("std/".to_owned() + name).into(), function);
        });
}

type FileRef = Option<File>;

pub const SLEEP: BuiltinFunction = |_, args| {
    let Object::Integer(millis) = args[0].as_ref() else {
        return new_type_error_with_pos("sleep", NUMBER.type_of(), 0);
    };

    thread::sleep(Duration::from_millis(*millis as u64));
    NULL.clone()
};

static OPEN_FILE: Lazy<Mutex<FileRef>> = Lazy::new(|| Mutex::new(None));

// TODO: Make this a format_args macro. then this function will just be a wrapper around format_args
// TODO: with that we can have a : fprint (file print)
// TODO: println and print and fprintln
/// Prints the arguments to stdout
pub const PRINT: BuiltinFunction = |_, args| {
    println!("{}", args.iter().map(|v| v.to_string()).collect::<String>());
    NULL.clone()
};

/// Prints the arguments to stdout in a debug format.
pub const DEBUG: BuiltinFunction = |_, args| {
    println!("{:?}", args);
    NULL.clone()
};

/// Prints the arguments to stdout in a pretty debug format.
pub const PDEBUG: BuiltinFunction = |_, args| {
    println!("{:#?}", args);
    NULL.clone()
};

/// Reads the current global file into a string
pub const READ_FILE: BuiltinFunction = |_, _| {
    let mut lock = OPEN_FILE.lock();

    if lock.is_none() {
        return Reference::new(Object::String("".into()));
    }

    let mut file = lock.take().unwrap();

    let mut string = String::new();

    file.read_to_string(&mut string)
        .expect("error reading file");

    file.seek(std::io::SeekFrom::Start(0))
        .expect("trying to seek file");

    let _ = lock.insert(file);

    Reference::new(Object::String(string.into()))
};

/// Returns the current working directory
pub const PWD: BuiltinFunction = |_, _| {
    Reference::new(Object::String(
        current_dir()
            .expect("to get current dir")
            .to_str()
            .expect("to convert pathbuf to str")
            .into(),
    ))
};

/// Returns the current global file descriptor as a string
pub const FILE: BuiltinFunction = |_, _| {
    let lock = OPEN_FILE.lock();

    if let Some(file) = lock.as_ref() {
        Reference::new(Object::String(format!("FILE[{:p}]", file).into()))
    } else {
        NULL.clone()
    }
};

/// Opens a file into the global file descriptor
pub const OPEN: BuiltinFunction = |_, args| {
    let len = args.len();
    if len != 1 {
        return new_args_len_error("open", &args, 1);
    }

    if let Some(err) = typecheck_args(
        "open",
        STRING.type_of(),
        |obj| !matches!(obj.as_ref(), Object::String(_)),
        &args,
    ) {
        return err;
    }

    let Object::String(inner) = args[0].as_ref() else {
        panic!("This should never happen");
    };

    let file = File::open(inner.as_ref());

    match file {
        Ok(file) => {
            *OPEN_FILE.lock() = Some(file);
        }
        Err(err) => {
            return Reference::new(Object::Error(format!("Error opening file: {}", err).into()));
        }
    };

    NULL.clone()
};

///Closes the global file descriptor
pub const CLOSE: BuiltinFunction = |_, _| {
    *OPEN_FILE.lock() = None;

    NULL.clone()
};
