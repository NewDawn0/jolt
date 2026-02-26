//! Jolt - WASM note-taking web app.

pub mod canvas;
mod render;
mod types;

use crate::types::WasmResult;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn info(s: &str);
}

/// Wrapper for `console.log`.
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}

/// Wrapper for `console.error`.
#[macro_export]
macro_rules! console_error {
    ($($t:tt)*) => ($crate::error(&format_args!($($t)*).to_string()))
}

/// Wrapper for `console.info`.
#[macro_export]
macro_rules! console_info {
    ($($t:tt)*) => ($crate::info(&format_args!($($t)*).to_string()))
}

/// WASM entry point. Auto-called on module load.
/// @NOTE: Sets panic hook and starts canvas render loop.
#[wasm_bindgen(start)]
pub fn start() -> WasmResult<()> {
    console_error_panic_hook::set_once();
    Ok(())
}
