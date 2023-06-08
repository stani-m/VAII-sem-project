use instant::Instant;
use nalgebra_glm as glm;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use winit::event::{ElementState, MouseButton, VirtualKeyCode};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::web::{WindowBuilderExtWebSys, WindowExtWebSys},
    window::WindowBuilder,
};

use crate::cube::Cube;
use crate::game::Game;
use color::Color;
use web_gl::WebGLContext;

use crate::model::Model;

mod color;
mod cube;
mod game;
mod gfx;
mod logging;
pub mod model;
mod web_gl;

const RESOLUTION_SCALE: usize = 1;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let html_window = web_sys::window().ok_or("Couldn't get window")?;
    let document = html_window.document().ok_or("Couldn't get document")?;
    let canvas = document
        .get_element_by_id("render_target")
        .ok_or("Render target not found")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let fps_indicator = document
        .get_element_by_id("fps")
        .ok_or("Fps indicator not found")?
        .dyn_into::<web_sys::HtmlSpanElement>()?;
    let score_indicator = document
        .get_element_by_id("score")
        .ok_or("Score indicator not found")?
        .dyn_into::<web_sys::HtmlSpanElement>()?;
    let session_storage = html_window.session_storage()?.unwrap();

    let pixel_ratio = html_window.device_pixel_ratio();
    let mut width = canvas.client_width() as f64 * pixel_ratio;
    let mut height = canvas.client_height() as f64 * pixel_ratio;
    let mut mouse_x = 0.0;

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

    window.canvas().focus().unwrap();

    let mut framebuffer = gfx::Framebuffer::new(
        width as usize / RESOLUTION_SCALE,
        height as usize / RESOLUTION_SCALE,
    );

    let mut game = Game::start(15, score_indicator);

    let mut pressed_keys = HashSet::new();
    let mut just_pressed_keys = HashSet::new();

    let zoom = glm::scaling(&glm::vec3(1.0, 1.0, 1.0));
    let view = glm::look_at(
        &glm::vec3(0.0, 2.0, 4.0),
        &glm::vec3(0.0, 0.5, 0.0),
        &glm::vec3(0.0, 1.0, 0f32),
    );
    let projection =
        glm::perspective_fov_zo(45_f32.to_radians(), width as f32, height as f32, 0.1, 100.0);
    let mut camera = projection * view * zoom;

    let program_start = Instant::now();
    let mut last_frame_time = program_start;
    let mut last_second = 0;

    let mut frames = 0u16;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if let Some(key) = input.virtual_keycode {
                    if input.state == ElementState::Pressed {
                        if pressed_keys.insert(key) {
                            just_pressed_keys.insert(key);
                        }
                    } else {
                        pressed_keys.remove(&key);
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if state == ElementState::Pressed && button == MouseButton::Left {
                    let location = (mouse_x / (width / 3.0)) as i8;
                    game.move_to(location);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                mouse_x = position.x;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
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
                    fps_indicator.set_inner_text(&frames.to_string());
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
                    camera = projection * view * zoom;
                }

                framebuffer.clear(Color::BLACK);

                if just_pressed_keys.contains(&VirtualKeyCode::A)
                    || just_pressed_keys.contains(&VirtualKeyCode::Left)
                {
                    game.move_left();
                }
                if just_pressed_keys.contains(&VirtualKeyCode::D)
                    || just_pressed_keys.contains(&VirtualKeyCode::Right)
                {
                    game.move_right();
                }
                game.advance(&delta_time);
                game.draw(&mut framebuffer, &camera);
                if game.check_collision() {
                    session_storage
                        .set_item("score", &game.score().to_string())
                        .unwrap();
                    html_window.location().replace("/game-over.html").unwrap();
                }

                context
                    .update_texture(
                        framebuffer.as_slice(),
                        framebuffer.width() as i32,
                        framebuffer.height() as i32,
                    )
                    .unwrap();

                context.draw();

                last_frame_time = current_frame_time;
                just_pressed_keys.clear();
            }
            _ => (),
        }
    });
}
