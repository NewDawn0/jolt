use crate::{render::webgl2::Pipeline, types::WasmResult};

pub struct App {
    pipeline: Pipeline,
}
impl App {
    pub fn new() -> WasmResult<Self> {
        Ok(Self {
            pipeline: Pipeline::new("Canvas")?,
        })
    }
}
