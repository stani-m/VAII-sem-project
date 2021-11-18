use crate::color::Color;
use itertools::Itertools;
use nalgebra_glm as glm;
use std::mem;
use std::ops::{Index, IndexMut};

pub struct Framebuffer {
    color: Vec<Color>,
    depth: Vec<f32>,
    width: usize,
    height: usize,
}

impl Index<[usize; 2]> for Framebuffer {
    type Output = Color;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let (x, y) = (index[0], index[1]);
        let index = self.calculate_index(x, y);
        &self.color[index]
    }
}

impl IndexMut<[usize; 2]> for Framebuffer {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let (x, y) = (index[0], index[1]);
        let index = self.calculate_index(x, y);
        &mut self.color[index]
    }
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            color: vec![Color::default(); size],
            depth: vec![f32::MAX; size],
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        let size = self.width * self.height;

        self.color.resize(size, Color::default());
        self.depth.resize(size, f32::MAX);
    }

    pub fn clear(&mut self, color: Color) {
        self.color.fill(color);
        self.depth.fill(f32::MAX);
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        bytemuck::cast_slice(self.color.as_slice())
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
    let data = transform_data(data, transform);
    let data = clip_lines(data.tuple_windows());
    let data = perspective_divide(data);
    let data = viewport_transform(
        data,
        framebuffer.width() as f32,
        framebuffer.height() as f32,
    );

    for (from, to) in data {
        let from = (from[0] as i32, from[1] as i32, from[2]);
        let to = (to[0] as i32, to[1] as i32, to[2]);
        draw_line(framebuffer, from, to, color);
    }
}

pub fn draw_line_list(
    framebuffer: &mut Framebuffer,
    data: &[glm::Vec3],
    transform: &glm::Mat4x4,
    color: Color,
) {
    let data = transform_data(data, transform);
    let data = clip_lines(data.tuples());
    let data = perspective_divide(data);
    let data = viewport_transform(
        data,
        framebuffer.width() as f32,
        framebuffer.height() as f32,
    );

    for (from, to) in data {
        let from = (from[0] as i32, from[1] as i32, from[2]);
        let to = (to[0] as i32, to[1] as i32, to[2]);
        draw_line(framebuffer, from, to, color);
    }
}

fn transform_data<'a>(
    data: &'a [glm::Vec3],
    transform: &'a glm::Mat4x4,
) -> impl Iterator<Item = glm::Vec4> + 'a {
    data.iter().map(move |point| {
        transform * glm::vec4(point[0], point[1], point[2], 1.0)
    })
}

// TODO: Rewrite these functions to operate on iterator items instead of iterators
fn clip_lines(
    data: impl Iterator<Item = (glm::Vec4, glm::Vec4)>,
) -> impl Iterator<Item = (glm::Vec4, glm::Vec4)> {
    data.filter(|&(a, b)| {
        let a_w = a[3];
        let b_w = b[3];
        !(0..3).any(|i| a[i].abs() > a_w && b[i].abs() > b_w)
    })
}

fn perspective_divide(
    data: impl Iterator<Item = (glm::Vec4, glm::Vec4)>,
) -> impl Iterator<Item = (glm::Vec4, glm::Vec4)> {
    data.map(|(a, b)| {
        let a_w = a[3];
        let b_w = b[3];
        (a / a_w, b / b_w)
    })
}

fn viewport_transform(
    data: impl Iterator<Item = (glm::Vec4, glm::Vec4)>,
    width: f32,
    height: f32,
) -> impl Iterator<Item = (glm::Vec3, glm::Vec3)> {
    let x_screen_transform = width / 2.0;
    let y_screen_transform = height / 2.0;

    data.map(move |(a, b)| {
        let a = glm::vec3(
            (a[0] + 1.0) * x_screen_transform,
            (a[1] + 1.0) * y_screen_transform,
            a[2],
        );
        let b = glm::vec3(
            (b[0] + 1.0) * x_screen_transform,
            (b[1] + 1.0) * y_screen_transform,
            b[2],
        );
        (a, b)
    })
}

fn draw_line(
    framebuffer: &mut Framebuffer,
    from: (i32, i32, f32),
    to: (i32, i32, f32),
    color: Color,
) {
    let (mut x0, mut y0, mut z0) = from;
    let (mut x1, mut y1, mut z1) = to;
    let run = x1 - x0;
    let rise = y1 - y0;
    let dive = z1 - z0;
    let width = framebuffer.width();
    let height = framebuffer.height();
    let width_range = 0..width as i32;
    let height_range = 0..height as i32;
    let depth_range = 0.0..1.0;
    let mut put_pixel_if_possible = |x: i32, y: i32, z: f32| {
        if width_range.contains(&x) && height_range.contains(&y) && depth_range.contains(&z) {
            let index = framebuffer.calculate_index(x as usize, y as usize);
            let depth = unsafe { framebuffer.depth.get_unchecked_mut(index) };
            if z <= *depth {
                *depth = z;
                unsafe {
                    *framebuffer.color.get_unchecked_mut(index) = color;
                }
            }
        }
    };
    if run == 0 {
        let z_delta = dive / rise as f32;
        let mut z = z0;
        if y0 > y1 {
            mem::swap(&mut y0, &mut y1);
            mem::swap(&mut z0, &mut z1);
        }
        if x0 >= 0 && x0 < width as i32 {
            for y in y0..=y1 {
                put_pixel_if_possible(x0, y, z);
                z += z_delta;
            }
        }
    } else {
        let m = rise as f32 / run as f32;
        let adjust = if m >= 0.0 { 1 } else { -1 };
        let mut offset = 0;
        if m < 1.0 && m > -1.0 {
            let delta = rise.abs() * 2;
            let mut threshold = run.abs();
            let threshold_inc = threshold * 2;
            let z_delta = dive / run as f32;
            let mut z = z0;
            let mut y;
            if x0 > x1 {
                mem::swap(&mut x0, &mut x1);
                mem::swap(&mut z0, &mut z1);
                y = y1;
            } else {
                y = y0;
            }
            for x in x0..=x1 {
                put_pixel_if_possible(x, y, z);
                z += z_delta;
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
            let z_delta = dive / rise as f32;
            let mut z = z0;
            let mut x;
            if y0 > y1 {
                mem::swap(&mut y0, &mut y1);
                mem::swap(&mut z0, &mut z1);
                x = x1;
            } else {
                x = x0;
            }
            for y in y0..=y1 {
                put_pixel_if_possible(x, y, z);
                z += z_delta;
                offset += delta;
                if offset >= threshold {
                    x = x.wrapping_add(adjust);
                    threshold += threshold_inc;
                }
            }
        }
    }
}
