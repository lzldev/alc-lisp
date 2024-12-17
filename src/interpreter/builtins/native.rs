use std::{
    env::current_dir,
    fs::File,
    io::{Read, Seek},
    sync::Mutex,
};

use once_cell::sync::Lazy;

use crate::interpreter::{objects::Object, Env, Reference, NULL, STRING};

use super::{errors::new_args_len_error, typecheck_args};

type FileRef = Option<File>;

static OPEN_FILE: Lazy<Mutex<FileRef>> = Lazy::new(|| Mutex::new(None));

pub fn add_native_builtins(env: &mut Env) {
    env.insert(
        "print".into(),
        Reference::new(Object::Builtin {
            function: |args| {
                println!("{}", args.iter().map(|v| v.to_string()).collect::<String>());
                return NULL.clone();
            },
        }),
    );

    env.insert(
        "pwd".into(),
        Reference::new(Object::Builtin {
            function: |_| {
                return Reference::new(Object::String(
                    current_dir()
                        .expect("to get current dir")
                        .to_str()
                        .expect("to convert pathbuf to str")
                        .to_owned(),
                ));
            },
        }),
    );

    env.insert(
        "debug".into(),
        Reference::new(Object::Builtin {
            function: |args| {
                println!("{:?}", args);
                return NULL.clone();
            },
        }),
    );

    env.insert(
        "open".into(),
        Reference::new(Object::Builtin {
            function: |args| {
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

                return NULL.clone();
            },
        }),
    );

    env.insert(
        "read_file".into(),
        Reference::new(Object::Builtin {
            function: |_| {
                let mut lock = OPEN_FILE.lock().unwrap();

                let file = lock.as_mut().expect("file not opened");

                let mut string = String::new();
                file.read_to_string(&mut string)
                    .expect("error reading file");

                file.seek(std::io::SeekFrom::Start(0))
                    .expect("trying to seek file");

                return Reference::new(Object::String(string));
            },
        }),
    );

    env.insert(
        "file".into(),
        Reference::new(Object::Builtin {
            function: |_| {
                let lock = OPEN_FILE.lock().unwrap();

                if let Some(file) = lock.as_ref() {
                    return Reference::new(Object::String(format!("FILE[{:p}]", file)));
                } else {
                    return NULL.clone();
                }
            },
        }),
    );
}
