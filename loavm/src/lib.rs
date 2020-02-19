use wasm_bindgen::prelude::*;
use crate::loa::bytecode::BytecodeEncodingRead;

extern crate console_error_panic_hook;
extern crate loa;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
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
