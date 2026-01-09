use crate::util::{Pos, WasmResult};
use helper::pub_fields;
use js_sys::Float32Array;
use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation,
    window,
};

macro_rules! shader_src {
    ($file:expr) => {
        include_str!(concat!("../../shader/", $file))
    };
}

#[wasm_bindgen]
pub struct GlRender {
    ctx: WebGl2RenderingContext,
    prog: WebGlProgram,
    unifs: Uniforms,
}

#[wasm_bindgen]
impl GlRender {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> WasmResult<Self> {
        let ctx = window()
            .ok_or("Window not found")?
            .document()
            .ok_or("Document not found")?
            .get_element_by_id(canvas_id)
            .ok_or("Canvas not found")?
            .dyn_into::<HtmlCanvasElement>()?
            .get_context("webgl2")?
            .ok_or("Webgl2 not supported")?
            .dyn_into::<WebGl2RenderingContext>()?;
        let prog = Self::new_program(&ctx)?;
        let unifs = Uniforms::new(&ctx, &prog)?;
        Ok(Self { ctx, prog, unifs })
    }
    pub fn render(&self) {
        let u = &self.unifs;
        self.ctx.clear_color(0.2, 0.1, 0.3, 1.0);
        self.ctx.uniform2f(Some(&u.offset), 0.0, 0.0);
        self.ctx.uniform1f(Some(&u.grid_size), 10.0);
        self.ctx
            .draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, 0, 4);
    }
    fn new_program(ctx: &WebGl2RenderingContext) -> WasmResult<WebGlProgram> {
        let prog = ctx
            .create_program()
            .ok_or("Unable to create webgl program object")?;
        const FRAG_SRC: &str = shader_src!("grid.frag");
        const VERT_SRC: &str = shader_src!("grid.vert");
        let frag = Self::compile_shader(&ctx, &FRAG_SRC, WebGl2RenderingContext::FRAGMENT_SHADER)?;
        let vert = Self::compile_shader(&ctx, &VERT_SRC, WebGl2RenderingContext::VERTEX_SHADER)?;
        ctx.attach_shader(&prog, &frag);
        ctx.attach_shader(&prog, &vert);
        ctx.link_program(&prog);
        // Exit if linking failed
        if !ctx
            .get_program_parameter(&prog, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            return Err(ctx
                .get_program_info_log(&prog)
                .ok_or("Unknown error linking program")?
                .into());
        }
        ctx.use_program(Some(&prog));
        // Intialize buffers
        let vert: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
        let pos_attr = ctx.get_attrib_location(&prog, "a_pos");
        let buf = ctx.create_buffer().ok_or("Failed to create buffer")?;
        ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buf));
        unsafe {
            let pos_arr_view = Float32Array::view(&vert);
            ctx.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &pos_arr_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        let vao = ctx
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        ctx.bind_vertex_array(Some(&vao));
        ctx.vertex_attrib_pointer_with_i32(
            pos_attr as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        ctx.enable_vertex_attrib_array(pos_attr as u32);
        ctx.bind_vertex_array(Some(&vao));
        Ok(prog)
    }
    fn compile_shader(
        ctx: &WebGl2RenderingContext,
        src: &str,
        shader_t: u32,
    ) -> WasmResult<WebGlShader> {
        let shader = ctx
            .create_shader(shader_t)
            .ok_or("Unable to create shader object")?;
        ctx.shader_source(&shader, src);
        ctx.compile_shader(&shader);
        match ctx
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            true => Ok(shader),
            false => Err(ctx
                .get_shader_info_log(&shader)
                .ok_or("Unknown error creating shader object")?
                .into()),
        }
    }
}

#[pub_fields]
pub struct Uniforms {
    offset: WebGlUniformLocation,
    grid_size: WebGlUniformLocation,
}
impl Uniforms {
    fn new(ctx: &WebGl2RenderingContext, prog: &WebGlProgram) -> WasmResult<Self> {
        let get_uniform = |attr| {
            ctx.get_uniform_location(&prog, attr)
                .ok_or(format!("Uniform {} not found", attr))
        };
        Ok(Self {
            offset: get_uniform("u_offset")?,
            grid_size: get_uniform("u_grid_size")?,
        })
    }
}
