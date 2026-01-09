use crate::{
    render::webgl2,
    util::{Pos, WasmResult},
};

pub struct Canvas {
    pos: Pos<isize>,
    render: webgl2::GlRender,
}
impl Canvas {
    pub fn new(canvas_id: &str) -> WasmResult<Self> {
        let render = webgl2::GlRender::new(canvas_id)?;
        Ok(Self {
            pos: Pos::default(),
            render,
        })
    }
    pub fn render(&self) {
        self.render.render();
    }
}
