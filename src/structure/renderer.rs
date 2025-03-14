use minifb::{Key, Window};

use crate::{Camera, Canvas, Direction, World};

pub struct Renderer {
    pub window: Window,
    pub canvas: Canvas,
    pub world: World,
    pub camera: Camera,
    pub size: (usize, usize),
}

impl Renderer {
    pub fn new(window: Window, canvas: Canvas, world: World, camera: Camera) -> Renderer {
        let size = window.get_size();

        Renderer {
            window,
            canvas,
            world,
            camera,
            size,
        }
    }

    pub fn render(&mut self, draw: fn(canvas: &mut Canvas, world: &World, camera: &Camera)) {
        while self.window.is_open() {
            if self.window.is_key_down(Key::Escape) {
                break;
            }

            let current_size = self.window.get_size();

            if self.size != current_size {
                self.size = current_size;

                self.canvas.resize(self.size.0, self.size.1);
                self.camera.resize(self.size.0 as f32 / self.size.1 as f32);

                self.size = (current_size.0, current_size.1);
            }

            if self.window.is_key_down(Key::A) {
                self.camera.translate(Direction::Left);
            }
            if self.window.is_key_down(Key::D) {
                self.camera.translate(Direction::Right);
            }
            if self.window.is_key_down(Key::W) {
                self.camera.translate(Direction::Forward);
            }
            if self.window.is_key_down(Key::S) {
                self.camera.translate(Direction::Backward);
            }

            if self.window.is_key_down(Key::Up) {
                self.camera.rotate_x(1.);
            }
            if self.window.is_key_down(Key::Down) {
                self.camera.rotate_x(-1.);
            }

            if self.window.is_key_down(Key::Right) {
                self.camera.rotate_y(1.);
            }
            if self.window.is_key_down(Key::Left) {
                self.camera.rotate_y(-1.);
            }

            self.camera.update();

            draw(&mut self.canvas, &self.world, &self.camera);

            let buffer = self.canvas.pixels();

            self.window
                .update_with_buffer(&buffer, self.size.0, self.size.1)
                .unwrap();

            // std::thread::sleep(Duration::from_millis(16));
        }
    }
}
