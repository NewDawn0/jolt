//! Type aliases for WASM interop.

use wasm_bindgen::prelude::*;

/// Result type for wasm-bindgen functions.
pub type WasmResult<T> = Result<T, JsValue>;

/// Vec2 type for viewport and position
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    /// Constructor
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    /// Shorthand for readability
    pub fn update(&mut self, x: f32, y: f32) {
        (self.x, self.y) = (x, y);
    }
}
