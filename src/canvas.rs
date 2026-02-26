//! Canvas management - WebGL2 init and render loop.

use std::sync::mpsc::{self, Sender};

use crate::{console_info, render::webgl2::Pipeline, types::WasmResult};
use wasm_bindgen_futures::spawn_local;

/// WebGL2 canvas wrapper.
pub struct Canvas {
    pipeline: Pipeline,
    tx: Option<Sender<()>>,
}

/// Embeds shader source at compile time.
macro_rules! shader_src {
    ($file:expr) => {
        include_str!(concat!("shader/", $file))
    };
}

impl Canvas {
    /// Creates new canvas
    pub fn new() -> WasmResult<Self> {
        let mut pipeline = Pipeline::new("Canvas")?;
        const FRAG: &str = shader_src!("grid.frag");
        const VERT: &str = shader_src!("grid.vert");
        pipeline.add_shaders(FRAG, VERT)?;
        Ok(Self { pipeline, tx: None })
    }
    /// Starts async render loop.
    pub fn start_render(&mut self) {
        let (tx, rx) = mpsc::channel::<()>();
        self.tx = Some(tx);
        spawn_local(async move {
            console_info!("Canvas render thread started");
            loop {
                if rx.try_recv().is_ok() {
                    break;
                }
            }
            console_info!("Canvas render thread terminated");
        });
    }
}

/// Cleans up the rendering thread on Canvas deletion
impl Drop for Canvas {
    fn drop(&mut self) {
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(());
        }
    }
}
