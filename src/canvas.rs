//! Canvas management - WebGL2 init and render loop.

use crate::{
    console_info,
    render::webgl2::Pipeline,
    types::{Vec2, WasmResult},
};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, Window, window};

/// Embeds shader source at compile time.
macro_rules! shader_src {
    ($file:expr) => {
        include_str!(concat!("shader/", $file))
    };
}

/// WebGL2 canvas wrapper.
#[wasm_bindgen]
pub struct Canvas {
    position: Vec2,
    pipeline: Pipeline,
    viewport: Vec2,
    window: Window,
}

#[wasm_bindgen]
impl Canvas {
    /// Creates new canvas
    // #[wasm_bindgen(constructor)]
    pub fn new() -> WasmResult<Self> {
        // Window and Canvas elem setup
        let window = window().expect("No window found");
        let viewport = Vec2::new(0.0, 0.0);
        let position = Vec2::new(0.0, 0.0);
        let canvas = window
            .document()
            .expect("Unable to get document")
            .get_element_by_id("Canvas")
            .expect("Unable to find Canvas")
            .dyn_into::<HtmlCanvasElement>()?;

        // Rendering pipeline setup
        let mut pipeline = Pipeline::new(canvas)?;
        const FRAG: &str = shader_src!("grid.frag");
        const VERT: &str = shader_src!("grid.vert");
        pipeline.add_shaders(FRAG, VERT)?;

        // Set width
        let mut out = Self {
            position,
            pipeline,
            viewport,
            window,
        };
        // Handle initial resize
        out.handle_resize()?;
        Ok(out)
    }

    /// Called by the JS
    pub fn handle_resize(&mut self) -> WasmResult<()> {
        let width = self.window.inner_width()?.as_f64().unwrap() as f32;
        let height = self.window.inner_height()?.as_f64().unwrap() as f32;
        self.viewport.update(width, height);
        self.pipeline.resize(Vec2::new(height, width));
        self.render();
        Ok(())
    }

    /// Called by JS's request animation frame render loop
    pub fn render(&self) {
        console_info!("Rendering");
        self.pipeline.render();
    }
}
