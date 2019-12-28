use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;
extern crate loa;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run(bytes: &[u8]) {
    let instructions = loa::generation::Instructions::from_bytes(bytes).unwrap();

    let mut vm = loa::vm::VM::new();
    if let Some(result) = vm.eval_pop::<()>(instructions) {
        log(format!("{}", result).as_str());
    }
}
