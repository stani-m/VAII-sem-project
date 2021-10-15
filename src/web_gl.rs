use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    WebGl2RenderingContext,
    WebGlBuffer,
    WebGlProgram,
    WebGlTexture,
    WebGlVertexArrayObject,
    WebGlUniformLocation,
    WebGlShader,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub struct WebGLContext {
    context: WebGl2RenderingContext,
    program: WebGlProgram,
    texture: WebGlTexture,
    sampler_uniform: WebGlUniformLocation,
    vao: WebGlVertexArrayObject,
    vertex_buffer: WebGlBuffer,
}

impl WebGLContext {
    const VERTEX_SHADER_SOURCE: &'static str = r#"#version 300 es

precision mediump float;

in vec2 position;

out vec2 texCoord;

void main() {
    texCoord = position / 2.0 + 0.5;
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

    const FRAGMENT_SHADER_SOURCE: &'static str = r#"#version 300 es

precision mediump float;

uniform sampler2D sampler;

in vec2 texCoord;

out vec4 fragColor;

void main() {
    fragColor = texture(sampler, texCoord);
}
"#;

    const VERTEX_DATA: [f32; 8] = [
        -1.0, -1.0,
        -1.0, 1.0,
        1.0, -1.0,
        // -1.0, 1.0,
        // 1.0, -1.0,
        1.0, 1.0,
    ];

    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Result<Self, JsValue> {
        let context = canvas
            .get_context("webgl2")?
            .ok_or(JsValue::from_str("Couldn't acquire context"))?
            .dyn_into::<WebGl2RenderingContext>()?;

        context.pixel_storei(WebGl2RenderingContext::UNPACK_ALIGNMENT, 1);

        let vertex_shader = Self::compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            Self::VERTEX_SHADER_SOURCE,
        )?;

        let fragment_shader = Self::compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            Self::FRAGMENT_SHADER_SOURCE,
        )?;

        let program = Self::link_program(&context, &vertex_shader, &fragment_shader)?;
        context.use_program(Some(&program));
        let position_attribute_location = context.get_attrib_location(&program, "position");

        let vao = context.create_vertex_array().ok_or(JsValue::from_str("Unable to create vao"))?;
        context.bind_vertex_array(Some(&vao));

        let vertex_buffer = context.create_buffer().ok_or(JsValue::from_str("Unable to create buffer"))?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        unsafe {
            let vertex_buffer_view = js_sys::Float32Array::view(&Self::VERTEX_DATA);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertex_buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        };

        context.vertex_attrib_pointer_with_i32(0, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        context.active_texture(WebGl2RenderingContext::TEXTURE0);
        let texture = context.create_texture().ok_or(JsValue::from_str("Unable to create texture"))?;
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::REPEAT as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::REPEAT as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );

        let sampler_uniform = context
            .get_uniform_location(&program, "sampler")
            .ok_or(JsValue::from_str("Unable to locate uniform"))?;

        context.uniform1i(Some(&sampler_uniform), 0);

        Ok(Self {
            context,
            program,
            texture,
            sampler_uniform,
            vao,
            vertex_buffer,
        })
    }

    pub fn update_texture(&self, data: &[u8], width: i32, height: i32) -> Result<(), JsValue> {
        self.context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGB as i32,
            width,
            height,
            0,
            WebGl2RenderingContext::RGB,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(data),
        )
    }

    pub fn resize(&self, new_width: i32, new_height: i32) {
        self.context.viewport(0, 0, new_width, new_height);
    }

    pub fn bind_all_objects(&self) {
        self.context.bind_vertex_array(Some(&self.vao));
        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.vertex_buffer));
        self.context.active_texture(WebGl2RenderingContext::TEXTURE0);
        self.context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.texture));
        self.context.use_program(Some(&self.program));
        self.context.uniform1i(Some(&self.sampler_uniform), 0);
    }

    pub fn draw(&self) {
        self.context.draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, 0, 4);
    }

    fn compile_shader(
        context: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, JsValue> {
        let shader = context
            .create_shader(shader_type)
            .ok_or(JsValue::from_str("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
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

    fn link_program(
        context: &WebGl2RenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, JsValue> {
        let program = context
            .create_program()
            .ok_or(JsValue::from_str("Unable to create shader object"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
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
