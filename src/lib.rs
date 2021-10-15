mod web_gl;

use web_gl::WebGLContext;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    dpi::PhysicalSize,
    platform::web::{WindowExtWebSys, WindowBuilderExtWebSys},
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let html_window = web_sys::window().ok_or(JsValue::from_str("Couldn't get window"))?;
    let document = html_window.document().ok_or(JsValue::from_str("Couldn't get document"))?;
    let canvas = document
        .get_element_by_id("render_target")
        .ok_or(JsValue::from_str("Render target not found"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let pixel_ratio = html_window.device_pixel_ratio();
    let mut width: f64 = html_window.inner_width()?.as_f64().unwrap() * pixel_ratio;
    let mut height: f64 = html_window.inner_height()?.as_f64().unwrap() * pixel_ratio;

    let event_loop = EventLoop::new();
    let window: Window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(width, height))
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .unwrap();

    let context = WebGLContext::new(&window.canvas())?;
    context.bind_all_objects();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {input, .. }, ..
            } => {
                if let Some(key) = input.virtual_keycode {
                    log(&format!("{:?}", key));
                }
            }
            Event::MainEventsCleared => {
                let new_width = html_window.inner_width().unwrap().as_f64().unwrap() * pixel_ratio;
                let new_height = html_window.inner_height().unwrap().as_f64().unwrap() * pixel_ratio;
                if new_width != width || new_height != height {
                    window.set_inner_size(PhysicalSize::new(new_width, new_height));
                    context.resize(new_width as i32, new_height as i32);
                    width = new_width;
                    height = new_height;
                    log(&format!("Resized to: {}, {}", width, height));
                }

                let texture = [
                    0, 0, 0, 255, 255, 255,
                    255, 255, 255, 0, 0, 0,
                ];
                context.update_texture(&texture, 2, 2).unwrap();
                context.draw();
            }
            _ => (),
        }
    });
}
