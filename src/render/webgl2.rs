//! WebGL2 pipeline - context, shaders, VAO.

use crate::types::WasmResult;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram as GLProgram,
    WebGlShader as GLShader, WebGlVertexArrayObject as GLVAO, window,
};

/// WebGL2 rendering pipeline.
pub struct Pipeline {
    ctx: GL,
    program: GLProgram,
    vao: GLVAO,
}
impl Pipeline {
    /// Creates pipeline from canvas element.
    pub fn new(canvas_id: &str) -> WasmResult<Self> {
        let ctx = window()
            .expect("No window found")
            .document()
            .expect("No document found")
            .get_element_by_id(canvas_id)
            .expect(&format!("Canvas elem `{}` not found", canvas_id))
            .dyn_into::<HtmlCanvasElement>()?
            .get_context("webgl2")?
            .expect("WebGl2 not supported")
            .dyn_into::<GL>()?;
        let program = ctx
            .create_program()
            .ok_or("Unable to create webgl program")?;
        let vao = ctx
            .create_vertex_array()
            .ok_or("Failed to create VAO for rendering")?;
        Ok(Self { ctx, program, vao })
    }
    /// Compiles and attaches shaders, links program, configures VAO.
    pub fn add_shaders(&mut self, frag_src: &str, vert_src: &str) -> WasmResult<()> {
        let frag = self.compile_shader(frag_src, GL::FRAGMENT_SHADER)?;
        let vert = self.compile_shader(vert_src, GL::VERTEX_SHADER)?;
        self.ctx.attach_shader(&self.program, &frag);
        self.ctx.attach_shader(&self.program, &vert);
        self.ctx.link_program(&self.program);
        if !self
            .ctx
            .get_program_parameter(&self.program, GL::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            return Err(self
                .ctx
                .get_program_info_log(&self.program)
                .unwrap_or_default()
                .into());
        }
        self.ctx.bind_vertex_array(Some(&self.vao));
        self.ctx.enable_vertex_attrib_array(0);
        self.ctx
            .vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        self.ctx.bind_vertex_array(None);
        self.ctx.use_program(Some(&self.program));
        Ok(())
    }
    fn compile_shader(&self, src: &str, shader_t: u32) -> WasmResult<GLShader> {
        let shader = self
            .ctx
            .create_shader(shader_t)
            .ok_or("Unable to create shader object")?;
        self.ctx.shader_source(&shader, src);
        self.ctx.compile_shader(&shader);
        if self
            .ctx
            .get_shader_parameter(&shader, GL::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(self
                .ctx
                .get_shader_info_log(&shader)
                .unwrap_or_default()
                .into())
        }
    }
}
