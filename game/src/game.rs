use crate::{gfx, glm, Color, Cube};
use rand::prelude::*;
use std::time::Duration;

const X_POSITIONS: [f32; 3] = [-1.1, 0.0, 1.1];

pub struct Game {
    player: Cube,
    player_target_x: i8,
    cubes: Vec<Cube>,
    speed: f32,
    spawn_y: f32,
    score: u32,
    score_indicator: web_sys::HtmlSpanElement,
    buffer: Vec<u8>,
    rng: rand::rngs::ThreadRng,
}

impl Game {
    pub fn start(n_cubes: usize, score_indicator: web_sys::HtmlSpanElement) -> Self {
        let (gltf, mut buffers, _) =
            gltf::import_slice(include_bytes!("../assets/cube.gltf")).unwrap();
        let buffer = buffers.swap_remove(0).to_vec();
        let mut cubes = Vec::with_capacity(n_cubes);

        let mut rng = rand::thread_rng();
        let mut y = 6.0;
        for _ in 0..n_cubes {
            let mut cube = Cube::new(&gltf, &buffer, Color::CYAN);
            cube.move_to(*X_POSITIONS.choose(&mut rng).unwrap() as f32, y);
            cubes.push(cube);
            y += rng.gen_range(6.0..10.0);
        }

        Self {
            player: Cube::new(&gltf, &buffer, Color::MAGENTA),
            player_target_x: 1,
            cubes,
            speed: 1.0,
            spawn_y: y,
            score: 0,
            score_indicator,
            buffer,
            rng,
        }
    }

    pub fn draw(&self, framebuffer: &mut gfx::Framebuffer, camera: &glm::Mat4) {
        for cube in &self.cubes {
            cube.draw(framebuffer, camera, &self.buffer);
        }
        self.player.draw(framebuffer, camera, &self.buffer);
    }

    pub fn advance(&mut self, delta_time: &Duration) {
        for cube in &mut self.cubes {
            cube.move_to(
                cube.x(),
                cube.y() - self.speed * delta_time.as_secs_f32() * 1.5,
            );
            if cube.y() < -3.0 {
                cube.move_to(*X_POSITIONS.choose(&mut self.rng).unwrap(), self.spawn_y);
                self.score += 1;
                self.score_indicator.set_inner_text(&self.score.to_string());
                self.speed += 0.1;
            }
        }
        let new_player_x = self.player.x()
            + (X_POSITIONS[self.player_target_x as usize] - self.player.x())
                * delta_time.as_secs_f32()
                * 5.0;
        self.player.move_to(new_player_x, self.player.y());
    }

    pub fn move_left(&mut self) {
        self.player_target_x = (self.player_target_x - 1).rem_euclid(3);
    }

    pub fn move_right(&mut self) {
        self.player_target_x = (self.player_target_x + 1).rem_euclid(3);
    }

    pub fn move_to(&mut self, location: i8) {
        self.player_target_x = location;
    }

    pub fn check_collision(&self) -> bool {
        for cube in &self.cubes {
            if self.player.collides_with(cube) {
                return true;
            }
        }
        return false;
    }

    pub fn score(&self) -> u32 {
        self.score
    }
}
