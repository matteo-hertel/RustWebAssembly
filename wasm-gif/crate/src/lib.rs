#[macro_use]
extern crate cfg_if;

extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// Definitions of the functionality available in JS, which wasm-bindgen will
// generate shims for today (and eventually these should be near-0 cost!)
//
// These definitions need to be hand-written today but the current vision is
// that we'll use WebIDL to generate this `extern` block into a crate which you
// can link and import. There's a tracking issue for this at
// https://github.com/rustwasm/wasm-bindgen/issues/42
//
// In the meantime these are written out by hand and correspond to the names and
// signatures documented on MDN, for example
#[wasm_bindgen]
extern "C" {
    type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;
    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    type Element;
    #[wasm_bindgen(method, setter = src)]
    fn set_src(this: &Element, src: &str);
    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner_html(this: &Element, html: &str);
    #[wasm_bindgen(method, js_name = appendChild)]
    fn append_child(this: &Element, other: Element);
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

macro_rules! log {
  ($($t:tt)*) => (log(&format!($($t)*)))
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() {
    //    let img = document.createElement("img");
    //    img.set_src("https://media.giphy.com/media/Nx0rz3jtxtEre/giphy.gif");
    let now = js_sys::Date::now();
    let now_date = js_sys::Date::new(&JsValue::from_f64(now));
    let val = document.createElement("p");
    val.set_inner_html(&format!(
        "Hello from Rust, it's {}:{}:{}!",
        now_date.get_hours(),
        now_date.get_minutes(),
        now_date.get_seconds()
    ));
    document.body().append_child(val);
    log!("{} is the word", "bird");
    //   document.body().append_child(img);
}
#[wasm_bindgen]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[wasm_bindgen]
pub struct Image {
    pixels: Vec<Color>,
}

#[wasm_bindgen]
impl Image {
    pub fn new() -> Image {
        let color1 = Color {
            red: 0,
            green: 255,
            blue: 0,
        };
        let color2 = Color {
            red: 0,
            green: 0,
            blue: 0,
        };
        let pixels = vec![color1, color2];
        Image { pixels }
    }

    pub fn pixels_ptr(&self) -> *const Color {
        self.pixels.as_ptr()
    }
}
