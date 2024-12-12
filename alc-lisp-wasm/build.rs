fn main() {
    let _message = DropMessage::new("=".repeat(60));

    if !std::env::var("_").is_ok_and(|v| v.contains("wasm-pack")) {
        println!("not in a wasm-pack build skipping ts_rs types export");
        return;
    };

    println!("Building alc-lisp-wasm");
    <alc_lisp::ast::Node as ts_rs::TS>::export_all_to("./pkg/types/")
        .expect("ts_rs::TS::export_all_to");
}

pub struct DropMessage {
    message: String,
    err: bool,
}

impl DropMessage {
    pub fn new(message: String) -> Self {
        println!("{}", message);
        DropMessage {
            message,
            err: false,
        }
    }

    pub fn new_err(message: String) -> Self {
        eprintln!("{}", message);
        DropMessage { message, err: true }
    }
}

impl Drop for DropMessage {
    fn drop(&mut self) {
        if self.err {
            eprintln!("{}", self.message);
        } else {
            println!("{}", self.message);
        }
    }
}
