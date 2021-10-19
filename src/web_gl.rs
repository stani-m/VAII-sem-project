use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    WebGlRenderingContext,
    WebGlBuffer,
    WebGlProgram,
    WebGlTexture,
    WebGlUniformLocation,
    WebGlShader,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub struct WebGLContext {
    context: WebGlRenderingContext,
    program: WebGlProgram,
    texture: WebGlTexture,
    sampler_uniform: WebGlUniformLocation,
    position_attrib_loc: i32,
    vertex_buffer: WebGlBuffer,
}

impl WebGLContext {
    const VERTEX_SHADER_SOURCE: &'static str = r#"
precision mediump float;

attribute vec2 position;

varying vec2 texCoord;

void main() {
    texCoord = position / 2.0 + 0.5;
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

    const FRAGMENT_SHADER_SOURCE: &'static str = r#"
precision mediump float;

uniform sampler2D sampler;

varying vec2 texCoord;

void main() {
    gl_FragColor = texture2D(sampler, texCoord);
}
"#;

    const VERTEX_DATA: [f32; 8] = [
        -1.0, -1.0,
        -1.0, 1.0,
        1.0, -1.0,
        1.0, 1.0,
    ];

    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Result<Self, JsValue> {
        let context = canvas
            .get_context("webgl")?
            .ok_or("Couldn't acquire context")?
            .dyn_into::<WebGlRenderingContext>()?;

        context.pixel_storei(WebGlRenderingContext::UNPACK_ALIGNMENT, 1);

        let vertex_shader = Self::compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            Self::VERTEX_SHADER_SOURCE,
        )?;

        let fragment_shader = Self::compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            Self::FRAGMENT_SHADER_SOURCE,
        )?;

        let program = Self::link_program(&context, &vertex_shader, &fragment_shader)?;
        context.use_program(Some(&program));
        let position_attrib_loc = context.get_attrib_location(&program, "position");

        let sampler_uniform = context
            .get_uniform_location(&program, "sampler")
            .ok_or("Unable to locate uniform")?;

        context.uniform1i(Some(&sampler_uniform), 0);

        let vertex_buffer = context.create_buffer().ok_or("Unable to create buffer")?;
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        unsafe {
            let vertex_buffer_view = js_sys::Float32Array::view(&Self::VERTEX_DATA);

            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vertex_buffer_view,
                WebGlRenderingContext::STATIC_DRAW,
            );
        };

        context.enable_vertex_attrib_array(position_attrib_loc as u32);
        context.vertex_attrib_pointer_with_i32(position_attrib_loc as u32, 2, WebGlRenderingContext::FLOAT, false, 0, 0);

        context.active_texture(WebGlRenderingContext::TEXTURE0);
        let texture = context.create_texture().ok_or("Unable to create texture")?;
        context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));
        context.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_S,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        context.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_T,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        context.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MIN_FILTER,
            WebGlRenderingContext::NEAREST as i32,
        );
        context.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MAG_FILTER,
            WebGlRenderingContext::NEAREST as i32,
        );

        Ok(Self {
            context,
            program,
            texture,
            sampler_uniform,
            position_attrib_loc,
            vertex_buffer,
        })
    }

    pub fn update_texture(&self, data: &[u8], width: i32, height: i32) -> Result<(), JsValue> {
        self.context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGlRenderingContext::TEXTURE_2D,
            0,
            WebGlRenderingContext::RGB as i32,
            width,
            height,
            0,
            WebGlRenderingContext::RGB,
            WebGlRenderingContext::UNSIGNED_BYTE,
            Some(data),
        )
    }

    pub fn resize(&self, width: i32, height: i32) {
        self.context.viewport(0, 0, width, height);
    }

    pub fn bind_all_objects(&self) {
        self.context.enable_vertex_attrib_array(self.position_attrib_loc as u32);
        self.context.vertex_attrib_pointer_with_i32(self.position_attrib_loc as u32, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.vertex_buffer));
        self.context.active_texture(WebGlRenderingContext::TEXTURE0);
        self.context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.texture));
        self.context.use_program(Some(&self.program));
        self.context.uniform1i(Some(&self.sampler_uniform), 0);
    }

    pub fn draw(&self) {
        self.context.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);
    }

    // Adapted from https://github.com/rustwasm/wasm-bindgen/tree/master/examples/webgl
    fn compile_shader(
        context: &WebGlRenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, JsValue> {
        let shader = context
            .create_shader(shader_type)
            .ok_or("Unable to create shader object")?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or("Unknown error creating shader".to_string())
                .into())
        }
    }

    // Adapted from https://github.com/rustwasm/wasm-bindgen/tree/master/examples/webgl
    fn link_program(
        context: &WebGlRenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, JsValue> {
        let program = context
            .create_program()
            .ok_or("Unable to create shader object")?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or("Unknown error creating program object".to_string())
                .into())
        }
    }
}
