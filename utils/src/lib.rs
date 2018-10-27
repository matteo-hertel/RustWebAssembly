extern {
    fn appendToBody(x: u32);
}

#[no_mangle]
pub extern fn run() {
    unsafe{
      appendToBody(42)
    }
}

#[no_mangle]
pub extern fn add_one(x: u32) -> u32 {
    x + 1
}

