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
    Env, Program, Reference, NULL, NUMBER, STRING,
};

use super::{
    errors::{new_args_len_error, new_type_error_with_pos},
    typecheck_args,
};

/// Adds the native builtins to a environment
pub fn add_native_builtins(env: &mut Env) {
    env.insert(
        "print".into(),
        Reference::new(Object::Builtin { function: print }),
    );

    env.insert(
        "debug".into(),
        Reference::new(Object::Builtin { function: debug }),
    );

    env.insert(
        "pdebug".into(),
        Reference::new(Object::Builtin { function: pdebug }),
    );

    env.insert(
        "pwd".into(),
        Reference::new(Object::Builtin { function: pwd }),
    );

    env.insert(
        "open".into(),
        Reference::new(Object::Builtin { function: open }),
    );

    env.insert(
        "close".into(),
        Reference::new(Object::Builtin { function: close }),
    );

    env.insert(
        "read_file".into(),
        Reference::new(Object::Builtin {
            function: read_file,
        }),
    );

    env.insert(
        "file".into(),
        Reference::new(Object::Builtin { function: file }),
    );

    env.insert(
        "sleep".into(),
        Reference::new(Object::Builtin { function: SLEEP }),
    );
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

/// Prints the arguments to stdout
fn print(_: &mut Program, args: Vec<Reference>) -> Reference {
    println!("{}", args.iter().map(|v| v.to_string()).collect::<String>());
    NULL.clone()
}

/// Prints the arguments to stdout in a debug format.
pub fn debug(_: &mut Program, args: Vec<Reference>) -> Reference {
    println!("{:?}", args);
    NULL.clone()
}

/// Prints the arguments to stdout in a pretty debug format.
pub fn pdebug(_: &mut Program, args: Vec<Reference>) -> Reference {
    println!("{:#?}", args);
    NULL.clone()
}

/// Reads the current global file into a string
pub fn read_file(_: &mut Program, _: Vec<Reference>) -> Reference {
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
}

/// Returns the current working directory
pub fn pwd(_: &mut Program, _: Vec<Reference>) -> Reference {
    Reference::new(Object::String(
        current_dir()
            .expect("to get current dir")
            .to_str()
            .expect("to convert pathbuf to str")
            .into(),
    ))
}

/// Returns the current global file descriptor as a string
pub fn file(_: &mut Program, _: Vec<Reference>) -> Reference {
    let lock = OPEN_FILE.lock();

    if let Some(file) = lock.as_ref() {
        Reference::new(Object::String(format!("FILE[{:p}]", file).into()))
    } else {
        NULL.clone()
    }
}

/// Opens a file into the global file descriptor
pub fn open(_: &mut Program, args: Vec<Reference>) -> Reference {
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
}

///Closes the global file descriptor
pub fn close(_: &mut Program, _: Vec<Reference>) -> Reference {
    *OPEN_FILE.lock() = None;

    NULL.clone()
}
