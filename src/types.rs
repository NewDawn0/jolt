//! Type aliases for WASM interop.

use wasm_bindgen::prelude::*;

/// Result type for wasm-bindgen functions.
pub type WasmResult<T> = Result<T, JsValue>;
