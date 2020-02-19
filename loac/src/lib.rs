use loa::server::Server;
use loa::{SourceKind, URI};
use wasm_bindgen::prelude::*;
use std::sync::Mutex;

extern crate console_error_panic_hook;
extern crate lazy_static;
extern crate loa;

use lazy_static::lazy_static;
use loa::generation::Generator;
use loa::bytecode::BytecodeEncodingRead;
use loa::vm::VM;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

lazy_static! {
    static ref SERVER: Mutex<Server> = Mutex::new(Server::new());
}

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run(mut bytes: &[u8]) {
    let instructions = bytes.deserialize().unwrap();

    let mut vm = loa::vm::VM::new();
    if let Some(result) = vm.eval_pop::<()>(instructions) {
        log(format!("{}", result).as_str());
    }
}

#[wasm_bindgen]
pub fn register(uri: &str, code: &str) {
    let inline = uri == "<inline>";
    let uri = URI::Exact(uri.to_string());
    let mut server = SERVER.lock().unwrap();

    if inline {
        server.set(uri, code.into(), SourceKind::REPLLine);
    } else {
        server.set(uri, code.into(), SourceKind::Module);
    }
}

#[wasm_bindgen]
pub fn compile() {
    let mut server = SERVER.lock().unwrap();

    log(format!("{:#?}", server.diagnostics()).as_str());

    let mut generator = Generator::new(&mut server.analysis);
    let instructions = generator.generate_all().unwrap().into();

    let mut vm = VM::new();
    if let Some(o) = vm.eval_pop::<()>(instructions) {
        log(format!("{}", o).as_str());
    }
}
