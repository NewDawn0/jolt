//! Type aliases for WASM interop.

use wasm_bindgen::prelude::*;

/// Result type for wasm-bindgen functions.
pub type WasmResult<T> = Result<T, JsValue>;

/// 2D vector for positions and sizes.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    /// X coordinate.
    pub x: f32,
    /// Y coordinate.
    pub y: f32,
}

impl Vec2 {
    /// Creates a new Vec2 with given coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    /// Updates the coordinates.
    pub fn update(&mut self, x: f32, y: f32) {
        (self.x, self.y) = (x, y);
    }
}
