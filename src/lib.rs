mod web_gl;
mod color;
mod logging;
mod gfx;

use web_gl::WebGLContext;
use color::Color;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    platform::web::{WindowExtWebSys, WindowBuilderExtWebSys},
};
use nalgebra_glm as glm;

use instant::Instant;

const RESOLUTION_SCALE: usize = 1;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let html_window = web_sys::window().ok_or("Couldn't get window")?;
    let document = html_window.document().ok_or("Couldn't get document")?;
    let canvas = document
        .get_element_by_id("render_target")
        .ok_or("Render target not found")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let fps = document
        .get_element_by_id("fps_number")
        .ok_or("Fps indicator not found")?
        .dyn_into::<web_sys::HtmlSpanElement>()?;

    let pixel_ratio = html_window.device_pixel_ratio();
    let mut width = canvas.client_width() as f64 * pixel_ratio;
    let mut height = canvas.client_height() as f64 * pixel_ratio;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .unwrap();

    window.canvas().style().remove_property("width")?;
    window.canvas().style().remove_property("height")?;
    window.canvas().set_width(width as u32);
    window.canvas().set_height(height as u32);


    let context = WebGLContext::new(&window.canvas())?;
    context.bind_all_objects();

    let mut framebuffer = gfx::Framebuffer::new(
        width as usize / RESOLUTION_SCALE,
        height as usize / RESOLUTION_SCALE,
    );

    let front = [
        glm::vec3(0.5, 0.5, -0.5),
        glm::vec3(0.5, -0.5, -0.5),
        glm::vec3(-0.5, -0.5, -0.5),
        glm::vec3(-0.5, 0.5, -0.5),
        glm::vec3(0.5, 0.5, -0.5),
    ];

    let back = [
        glm::vec3(0.5, 0.5, 0.5),
        glm::vec3(0.5, -0.5, 0.5),
        glm::vec3(-0.5, -0.5, 0.5),
        glm::vec3(-0.5, 0.5, 0.5),
        glm::vec3(0.5, 0.5, 0.5),
    ];

    let sides = [
        glm::vec3(0.5, 0.5, -0.5),
        glm::vec3(0.5, 0.5, 0.5),
        glm::vec3(-0.5, 0.5, -0.5),
        glm::vec3(-0.5, 0.5, 0.5),
        glm::vec3(-0.5, -0.5, -0.5),
        glm::vec3(-0.5, -0.5, 0.5),
        glm::vec3(0.5, -0.5, -0.5),
        glm::vec3(0.5, -0.5, 0.5),
    ];

    let mut model = glm::identity();

    let view = glm::look_at(
        &glm::vec3(1.0, 2.0, 3.0),
        &glm::vec3(0.0, 0.0, 0.0),
        &glm::vec3(0.0, 1.0, 0f32),
    );
    let projection = glm::perspective_fov_zo(
        45_f32.to_radians(),
        width as f32,
        height as f32,
        0.1,
        100.0,
    );
    let mut camera = projection * view;


    let program_start = Instant::now();
    let mut last_frame_time = program_start;
    let mut last_second = 0;

    let mut frames = 0u16;

    event_loop.run(move |event, _, control_flow| {
        // *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {input, .. }, ..
            } => {
                if let Some(key) = input.virtual_keycode {
                    console_log!("{:?}", key);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_), ..
            } => {
                window.canvas().style().remove_property("width").unwrap();
                window.canvas().style().remove_property("height").unwrap();
            }
            Event::MainEventsCleared => {
                let current_frame_time = Instant::now();
                let delta_time = current_frame_time - last_frame_time;
                let since_program_start = current_frame_time - program_start;

                let current_second = since_program_start.as_secs();
                if current_second != last_second {
                    fps.set_inner_text(&frames.to_string());
                    last_second = current_second;
                    frames = 0;
                }
                frames += 1;

                let new_width = window.canvas().client_width() as f64 * pixel_ratio;
                let new_height = window.canvas().client_height() as f64 * pixel_ratio;
                if new_width != width || new_height != height {
                    width = new_width;
                    height = new_height;

                    window.canvas().set_width(width as u32);
                    window.canvas().set_height(height as u32);
                    context.resize(width as i32, height as i32);
                    framebuffer.resize(
                        width as usize / RESOLUTION_SCALE,
                        height as usize / RESOLUTION_SCALE,
                    );
                    let projection = glm::perspective_fov_zo(
                        45_f32.to_radians(),
                        width as f32,
                        height as f32,
                        0.1,
                        100.0,
                    );
                    camera = projection * view;
                }

                model = glm::rotate_y(&model, delta_time.as_secs_f32() * -std::f32::consts::PI / 8.0);
                let transform = camera * model;

                framebuffer.clear(Color::BLACK);

                gfx::draw_line_strip(&mut framebuffer, &front, &transform, Color::CYAN);
                gfx::draw_line_strip(&mut framebuffer, &back, &transform, Color::YELLOW);
                gfx::draw_line_list(&mut framebuffer, &sides, &transform, Color::MAGENTA);

                context.update_texture(
                    framebuffer.as_slice(),
                    framebuffer.width() as i32,
                    framebuffer.height() as i32
                ).unwrap();

                context.draw();

                last_frame_time = current_frame_time;
            }
            _ => (),
        }
    });
}
