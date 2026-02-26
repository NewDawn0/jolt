//! WebGL2 pipeline - context, shaders, VAO.

use crate::types::{Vec2, WasmResult};
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlBuffer, WebGlProgram as GLProgram,
    WebGlShader as GLShader, WebGlUniformLocation, WebGlVertexArrayObject as GLVAO, window,
};

/// WebGL2 rendering pipeline.
pub struct Pipeline {
    /// WebGL2 rendering context.
    ctx: GL,
    /// Compiled shader program.
    program: GLProgram,
    /// Vertex array object.
    vao: GLVAO,
    /// Vertex buffer object.
    vbo: WebGlBuffer,
    /// Viewport uniform location.
    u_viewport: Option<WebGlUniformLocation>,
    /// Position uniform location.
    u_position: Option<WebGlUniformLocation>,
    /// Canvas element.
    canvas: HtmlCanvasElement,
}

impl Pipeline {
    /// Creates pipeline from canvas element.
    pub fn new(canvas: HtmlCanvasElement) -> WasmResult<Self> {
        let ctx = canvas
            .get_context("webgl2")?
            .expect("WebGl2 not supported")
            .dyn_into::<GL>()?;
        let program = ctx
            .create_program()
            .ok_or("Unable to create webgl program")?;
        let vao = ctx
            .create_vertex_array()
            .ok_or("Failed to create VAO for rendering")?;
        let vbo = ctx
            .create_buffer()
            .ok_or("Failed to create VBO for rendering")?;
        Ok(Self {
            canvas,
            ctx,
            program,
            u_position: None,
            u_viewport: None,
            vao,
            vbo,
        })
    }

    /// Resizes canvas and updates viewport uniform.
    pub fn resize(&mut self, size: Vec2) {
        self.canvas.set_width(size.x as u32);
        self.canvas.set_height(size.y as u32);
        if let Some(loc) = &self.u_viewport {
            self.ctx.uniform2f(Some(loc), size.x, size.y);
        }
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

        // Create full-screen quad vertices (triangle strip)
        let vertices: [f32; 8] = [
            -1.0, -1.0, // bottom left
            1.0, -1.0, // bottom right
            -1.0, 1.0, // top left
            1.0, 1.0, // top right
        ];

        self.ctx.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vbo));
        unsafe {
            let vertices_slice = std::slice::from_raw_parts(vertices.as_ptr(), vertices.len());
            self.ctx.buffer_data_with_array_buffer_view(
                GL::ARRAY_BUFFER,
                &js_sys::Float32Array::view(vertices_slice),
                GL::STATIC_DRAW,
            );
        }

        self.ctx.enable_vertex_attrib_array(0);
        self.ctx
            .vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        self.ctx.bind_buffer(GL::ARRAY_BUFFER, None);
        self.ctx.bind_vertex_array(None);
        self.ctx.use_program(Some(&self.program));

        self.u_viewport = self.ctx.get_uniform_location(&self.program, "u_viewport");
        self.u_position = self.ctx.get_uniform_location(&self.program, "u_position");

        Ok(())
    }

    /// Sets the position uniform (x, y offset)
    pub fn set_position(&mut self, position: Vec2) {
        if let Some(loc) = &self.u_position {
            self.ctx.uniform2f(Some(loc), position.x, position.y);
        }
    }

    /// Renders a frame
    pub fn render(&self) {
        self.ctx.clear_color(0.1, 0.1, 0.1, 1.0);
        self.ctx.clear(GL::COLOR_BUFFER_BIT);
        self.ctx.draw_arrays(GL::TRIANGLE_STRIP, 0, 4);
    }

    /// Compiles a shader from source.
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
