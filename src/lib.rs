mod canvas;
mod render;
mod util;
use canvas::Canvas;
use console_error_panic_hook;
use wasm_bindgen::prelude::*;

use crate::util::WasmResult;

#[wasm_bindgen(start)]
pub fn main() -> WasmResult<()> {
    console_error_panic_hook::set_once();
    let canvas = Canvas::new("Canvas")?;
    canvas.render();
    Ok(())
}
