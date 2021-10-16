use std::ops::{Index, IndexMut, Range};
use itertools::Itertools;
use nalgebra_glm as glm;
use crate::color::Color;

pub struct Framebuffer {
    data: Vec<Color>,
    width: usize,
    height: usize,
}

impl Index<[usize; 2]> for Framebuffer {
    type Output = Color;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let (x, y) = (index[0], index[1]);
        let index = self.calculate_index(x, y);
        &self.data[index]
    }
}

impl IndexMut<[usize; 2]> for Framebuffer {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let (x, y) = (index[0], index[1]);
        let index = self.calculate_index(x, y);
        &mut self.data[index]
    }
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![Color::default(); width * height],
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.data.resize(width * height, Color::default());
        self.width = width;
        self.height = height;
    }

    pub fn clear(&mut self, color: Color) {
        self.data.fill(color);
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        bytemuck::cast_slice(self.data.as_slice())
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    fn calculate_index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
}

pub fn draw_line_strip(
    framebuffer: &mut Framebuffer,
    data: &[glm::Vec3],
    transform: &glm::Mat4x4,
    color: Color,
) {
    let data = transform_data(
        data,
        framebuffer.width() as f32,
        framebuffer.height() as f32,
        transform
    );

    for (from, to) in data.tuple_windows() {
        let from = (from[0] as i32, from[1] as i32);
        let to = (to[0] as i32, to[1] as i32);
        draw_line(framebuffer, from, to, color);
    }
}

pub fn draw_line_list(
    framebuffer: &mut Framebuffer,
    data: &[glm::Vec3],
    transform: &glm::Mat4x4,
    color: Color,
) {
    let data = transform_data(
        data,
        framebuffer.width() as f32,
        framebuffer.height() as f32,
        transform,
    );

    for (from, to) in data.tuples() {
        let from = (from[0] as i32, from[1] as i32);
        let to = (to[0] as i32, to[1] as i32);
        draw_line(framebuffer, from, to, color);
    }
}

fn transform_data<'a>(
    data: &'a [glm::Vec3],
    width: f32,
    height: f32,
    transform: &'a glm::Mat4x4,
) -> impl Iterator<Item = glm::Vec2> + 'a {
    let x_screen_transform = width / 2.0;
    let y_screen_transform = height / 2.0;

    data
        .iter()
        .map(move |point| {
            let transformed_point = transform * glm::vec4(point[0], point[1], point[2], 1.0);
            let w = transformed_point[3];
            let clip_point = transformed_point / w;
            let screen_point = glm::vec2(
                (clip_point[0] + 1.0) * x_screen_transform,
                (clip_point[1] + 1.0) * y_screen_transform,
            );
            screen_point
        })
}

fn draw_line(framebuffer: &mut Framebuffer, from: (i32, i32), to: (i32, i32), color: Color) {
    let (mut x0, mut y0) = from;
    let (mut x1, mut y1) = to;
    let (run, rise) = (x1 - x0, y1 - y0);
    if run == 0 {
        if y0 > y1 {
            let temp = y0;
            y0 = y1;
            y1 = temp;
        }
        if x0 >= 0 && x0 < framebuffer.width() as i32 {
            let height_range = 0..framebuffer.height() as i32;

            fn intersect(a: Range<i32>, b: Range<i32>) -> Range<i32> {
                a.start.max(b.start)..a.end.min(b.end)
            }

            for y in intersect(y0..y1 + 1, height_range) {
                let index = framebuffer.calculate_index(x0 as usize, y as usize);
                unsafe { *framebuffer.data.get_unchecked_mut(index) = color; }
            }
        }
    } else {
        let width_range = 0..framebuffer.width() as i32;
        let height_range = 0..framebuffer.height() as i32;
        let mut put_pixel_if_possible = | x: i32, y: i32 | {
            if width_range.contains(&x) && height_range.contains(&y) {
                let index = framebuffer.calculate_index(x as usize, y as usize);
                unsafe { *framebuffer.data.get_unchecked_mut(index) = color; }
            }
        };
        let m = rise as f32 / run as f32;
        let adjust = if m >= 0.0 { 1 } else { -1 };
        let mut offset = 0;
        if m < 1.0 && m > -1.0 {
            let delta = rise.abs() * 2;
            let mut threshold = run.abs();
            let threshold_inc = threshold * 2;
            let mut y;
            if x0 > x1 {
                let temp = x0;
                x0 = x1;
                x1 = temp;
                y = y1;
            } else {
                y = y0;
            }
            for x in x0..=x1 {
                put_pixel_if_possible(x, y);
                offset += delta;
                if offset >= threshold {
                    y = y + adjust;
                    threshold += threshold_inc;
                }
            }
        } else {
            let delta = run.abs() * 2;
            let mut threshold = rise.abs();
            let threshold_inc = threshold * 2;
            let mut x;
            if y0 > y1 {
                let temp = y0;
                y0 = y1;
                y1 = temp;
                x = x1;
            } else {
                x = x0;
            }
            for y in y0..=y1 {
                put_pixel_if_possible(x, y);
                offset += delta;
                if offset >= threshold {
                    x = x.wrapping_add(adjust);
                    threshold += threshold_inc;
                }
            }
        }
    }
}
