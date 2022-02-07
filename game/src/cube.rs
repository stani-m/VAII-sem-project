use crate::{gfx, glm, Color, Model};

pub struct Cube {
    model: Model,
    x: f32,
    y: f32,
}

impl Cube {
    pub fn new(gltf: &gltf::Document, buffer: &[u8], color: Color) -> Self {
        let cube_node = gltf
            .nodes()
            .find(|node| node.name() == Some("Cube"))
            .unwrap();
        let model = Model::from_node(&cube_node, buffer, color);
        Cube {
            model,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.model.set_translation(glm::vec3(self.x, 0.0, -self.y));
    }

    pub fn collides_with(&self, other: &Cube) -> bool {
        (self.x - other.x).abs() < 1.0 && (self.y - other.y).abs() < 1.0
    }

    pub fn draw(&self, framebuffer: &mut gfx::Framebuffer, camera: &glm::Mat4, buffer: &[u8]) {
        self.model.draw(framebuffer, camera, buffer);
    }
}
