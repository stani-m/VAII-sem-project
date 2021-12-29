use std::collections::HashSet;
use std::mem::size_of;

use gltf::json::accessor::ComponentType;
use itertools::Itertools;
use nalgebra_glm as glm;

use crate::{gfx, Color};

pub struct BufferView {
    offset: usize,
    length: usize,
}

impl BufferView {
    fn look(&self, buffer: &[u8]) -> &[glm::Vec3] {
        unsafe {
            std::slice::from_raw_parts(
                buffer[self.offset..(self.offset + self.length)].as_ptr() as *const glm::Vec3,
                self.length / size_of::<glm::Vec3>(),
            )
        }
    }
}

fn triangles_to_lines_index(triangles: &[u32]) -> Vec<u32> {
    let mut lines = HashSet::new();
    for (&a, &b, &c) in triangles.into_iter().tuples() {
        let mut a = a;
        let mut b = b;
        let mut c = c;
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        if a > c {
            std::mem::swap(&mut a, &mut c);
        }
        if b > c {
            std::mem::swap(&mut b, &mut c);
        }
        lines.insert([a, b]);
        lines.insert([b, c]);
        lines.insert([c, a]);
    }
    lines
        .into_iter()
        .flat_map(|line| line.into_iter())
        .collect()
}

pub struct Model {
    name: String,
    color: Color,
    vertex_buffer_view: BufferView,
    index_buffer: Vec<u32>,
    translation: glm::Vec3,
    rotation: glm::Quat,
    scale: glm::Vec3,
    children: Vec<Model>,
}

impl Model {
    pub fn from(gltf: &gltf::Document, buffer: &[u8], color: Color) -> Self {
        let scene = gltf.default_scene().unwrap();
        let node = scene.nodes().nth(0).unwrap();
        Self::from_node(&node, buffer, color)
    }

    pub fn from_node(node: &gltf::Node, buffer: &[u8], color: Color) -> Self {
        let name = node.name().unwrap().to_string();
        if node.mesh().unwrap().primitives().len() != 1 {
            panic!("Multiple primitives not supported!");
        }
        let primitive = node.mesh().unwrap().primitives().nth(0).unwrap();
        let vertex_view = primitive
            .attributes()
            .find_map(|attribute| match attribute.0 {
                gltf::Semantic::Positions => Some(attribute.1),
                _ => None,
            })
            .expect("Position attribute not found!")
            .view()
            .unwrap();
        let vertex_buffer_view = BufferView {
            offset: vertex_view.offset(),
            length: vertex_view.length(),
        };
        let index_accessor = primitive.indices().unwrap();
        let index_view = index_accessor.view().unwrap();
        let index_buffer = match index_accessor.data_type() {
            ComponentType::U8 => {
                let index_buffer = buffer
                    [index_view.offset()..(index_view.offset() + index_view.length())]
                    .into_iter()
                    .map(|&index| index as u32)
                    .collect_vec();
                triangles_to_lines_index(&index_buffer)
            }
            ComponentType::U16 => {
                let index_buffer = bytemuck::cast_slice::<u8, u16>(
                    &buffer[index_view.offset()..(index_view.offset() + index_view.length())],
                )
                .into_iter()
                .map(|&index| index as u32)
                .collect_vec();
                triangles_to_lines_index(&index_buffer)
            }
            ComponentType::U32 => triangles_to_lines_index(bytemuck::cast_slice(
                &buffer[index_view.offset()..(index_view.offset() + index_view.length())],
            )),
            _ => panic!("Unsupported index accessor data type!"),
        };
        let (translation, rotation, scale) = node.transform().decomposed();
        let translation = glm::make_vec3(&translation);
        let rotation = glm::make_quat(&rotation);
        let scale = glm::make_vec3(&scale);
        let children = node
            .children()
            .map(|child| Model::from_node(&child, buffer, color))
            .collect_vec();

        Self {
            name,
            color,
            vertex_buffer_view,
            index_buffer,
            translation,
            rotation,
            scale,
            children,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_child_color(&mut self, name: &str, color: Color) {
        if self.name == name {
            self.set_color(color);
        }
        for child in &mut self.children {
            child.set_child_color(name, color);
        }
    }

    pub fn set_translation(&mut self, translation: &glm::Vec3) {
        self.translation = *translation;
    }

    pub fn set_rotation(&mut self, rotation: &glm::Quat) {
        self.rotation = *rotation;
    }

    pub fn set_scale(&mut self, scale: &glm::Vec3) {
        self.scale = *scale;
    }

    pub fn translation(&self) -> glm::Vec3 {
        self.translation
    }

    pub fn rotation(&self) -> glm::Quat {
        self.rotation
    }

    pub fn scale(&self) -> glm::Vec3 {
        self.scale
    }

    pub fn model_matrix(&self) -> glm::Mat4 {
        glm::translation(&self.translation)
            * glm::quat_to_mat4(&self.rotation)
            * glm::scaling(&self.scale)
    }

    pub fn draw(&self, framebuffer: &mut gfx::Framebuffer, camera: &glm::Mat4, buffer: &[u8]) {
        let transform = camera * self.model_matrix();
        gfx::draw_line_list_indexed(
            framebuffer,
            self.vertex_buffer_view.look(buffer),
            &self.index_buffer,
            &transform,
            self.color,
        );
        for child in &self.children {
            child.draw(framebuffer, &transform, buffer);
        }
    }
}
