extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "../lib/domUtils")]
extern {
    fn appendToBody(s: &str);
    fn appendImage(s: &str);
}

#[wasm_bindgen]
pub extern fn run() {
    appendImage("https://media.giphy.com/media/Nx0rz3jtxtEre/giphy.gif");
    appendToBody("General Kenobi!");
}

#[wasm_bindgen]
pub extern fn add_one(x: u32) -> u32 {
    x + 1
}
