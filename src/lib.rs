mod canvas;
use console_error_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    panic!("Hello world");
    Ok(())
}
