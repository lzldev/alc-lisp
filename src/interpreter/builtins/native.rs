//! Builtin functions to be used in a native environment

use std::{
    env::current_dir,
    fs::File,
    io::{Read, Seek},
    sync::Mutex,
};

use once_cell::sync::Lazy;

use crate::interpreter::{objects::Object, Env, Reference, NULL, STRING};

use super::{errors::new_args_len_error, typecheck_args};

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
}

type FileRef = Option<File>;

static OPEN_FILE: Lazy<Mutex<FileRef>> = Lazy::new(|| Mutex::new(None));

/// Prints the arguments to stdout
fn print(args: Vec<Reference>) -> Reference {
    println!("{}", args.iter().map(|v| v.to_string()).collect::<String>());
    NULL.clone()
}

/// Prints the arguments to stdout in a debug format.
pub fn debug(args: Vec<Reference>) -> Reference {
    println!("{:?}", args);
    NULL.clone()
}

/// Prints the arguments to stdout in a pretty debug format.
pub fn pdebug(args: Vec<Reference>) -> Reference {
    println!("{:#?}", args);
    NULL.clone()
}

/// Reads the current global file into a string
pub fn read_file(_: Vec<Reference>) -> Reference {
    let mut lock = OPEN_FILE.lock().unwrap();

    let file = lock.as_mut().expect("file not opened");

    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("error reading file");

    file.seek(std::io::SeekFrom::Start(0))
        .expect("trying to seek file");

    Reference::new(Object::String(string))
}

/// Returns the current working directory
pub fn pwd(_: Vec<Reference>) -> Reference {
    Reference::new(Object::String(
        current_dir()
            .expect("to get current dir")
            .to_str()
            .expect("to convert pathbuf to str")
            .to_owned(),
    ))
}

/// Returns the current global file descriptor as a string
pub fn file(_: Vec<Reference>) -> Reference {
    let lock = OPEN_FILE.lock().unwrap();

    if let Some(file) = lock.as_ref() {
        Reference::new(Object::String(format!("FILE[{:p}]", file)))
    } else {
        NULL.clone()
    }
}

/// Opens a file into the global file descriptor
pub fn open(args: Vec<Reference>) -> Reference {
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

    let file = File::open(inner).expect("error trying to open file");

    *OPEN_FILE.lock().unwrap() = Some(file);

    NULL.clone()
}

///Closes the global file descriptor
pub fn close(_: Vec<Reference>) -> Reference {
    *OPEN_FILE.lock().unwrap() = None;

    NULL.clone()
}
