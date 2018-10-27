extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "../lib/domUtils")]
extern {
    fn appendToBody(s: &str);
}

#[wasm_bindgen]
pub extern fn run() {
    appendToBody("Hello There");
    appendToBody("General Kenobi!");
}

#[wasm_bindgen]
pub extern fn add_one(x: u32) -> u32 {
    x + 1
}
