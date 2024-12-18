use std::{
    env::{self},
    fs::{self, read_dir},
    io::Write,
    sync::LazyLock,
};

static SEPARATOR: LazyLock<String> = LazyLock::new(|| "=".repeat(60));

fn main() {
    let _message = DropMessage::new(&SEPARATOR);

    if !std::env::var("_").is_ok_and(|v| v.contains("wasm-pack")) {
        println!("not in a wasm-pack build skipping ts_rs types export");
        return;
    };

    let types_dir = "./pkg/types/";
    let file_ext_dir = env::var("OUT_DIR").unwrap() + "/types.ts";

    println!("Building alc-lisp-wasm");
    <alc_lisp::ast::Node as ts_rs::TS>::export_all_to(types_dir).expect("ts_rs::TS::export_all_to");
    <alc_lisp::interpreter::objects::Object as ts_rs::TS>::export_all_to(types_dir)
        .expect("ts_rs::TS::export_all_to");

    let mut out = fs::File::create(&file_ext_dir).expect("to open output file");

    let types_dir = read_dir(types_dir).expect("to open types dir");

    {
        let _out_message = DropMessage::new(&SEPARATOR);
        println!("Type Exports: ");
    }

    for file in types_dir.into_iter() {
        let path = file.expect("to open file").path();
        let name = path.file_stem().expect("to get file name");

        let export = format!(
            "export {{{}}} from './types/{}'",
            name.to_string_lossy(),
            path.file_name()
                .expect("to get file name")
                .to_string_lossy()
        );

        println!("{}", export);

        out.write_all(export.as_bytes()).expect("to write to file");
        out.write_all(b"\n").expect("to write to file");
    }

    println!("types written to : [{}]", &file_ext_dir);
}

pub struct DropMessage<'m> {
    message: &'m String,
    err: bool,
}

impl<'m> DropMessage<'m> {
    pub fn new(message: &'m String) -> Self {
        println!("{}", message);
        DropMessage {
            message,
            err: false,
        }
    }

    pub fn new_err(message: &'m String) -> Self {
        eprintln!("{}", message);
        DropMessage { message, err: true }
    }
}

impl Drop for DropMessage<'_> {
    fn drop(&mut self) {
        if self.err {
            eprintln!("{}", self.message);
        } else {
            println!("{}", self.message);
        }
    }
}
