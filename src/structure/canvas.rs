
use minifb::{Window, WindowOptions};
use std::fs::File;
use std::io::{self, Write};
use crate::{Color, Vector};
use crate::Point;

use std::time::Duration;
use minifb::Key;

use super::light::Light;
use super::objects::Material;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
    pub window: Window,
    pub camera_origin: Point,
    pub wall: f32,
}

impl Canvas {
    // handle negative values ?
    pub fn new(width: usize, height: usize, camera_origin: Point, wall: f32) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new([0.0, 0.0, 0.0]); width * height],
            window: Window::new(
                "My Canvas - ESC to quit",
                width,
                height,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| {
                panic!("cant create window : {}", e);
            }),
            camera_origin: camera_origin,
            wall: wall,
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.pixels[index] = color;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let index = y * self.width + x;
        self.pixels[index]
    }

    pub fn get_pixel_buffer(&self) -> Vec<u32> {
        let buffer: Vec<u32> = self.pixels.iter().map(|c| color_to_u32(c)).collect();
        buffer
    }

    pub fn update_window(&mut self) {
        // let mut buffer: Vec<u32> = self.pixels.iter().map(|c| color_to_u32(c)).collect();
        let buffer = self.get_pixel_buffer();
        self.window.update_with_buffer(&buffer, self.width, self.height).unwrap();
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = &self.pixels[y * self.width + x];

                // Clamp des valeurs entre 0 et 255
                let r = (pixel.red().clamp(0.0, 1.0) * 255.0) as u32;
                let g = (pixel.green().clamp(0.0, 1.0) * 255.0) as u32;
                let b = (pixel.blue().clamp(0.0, 1.0) * 255.0) as u32;

                ppm.push_str(&format!("{} {} {} ", r, g, b));
            }
            ppm.push('\n');
        }

        ppm
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        let content = self.to_ppm();
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn canvas_loop(&mut self, draw: fn(&mut Canvas), sleep: u64) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {

            draw(self);
            self.update_window();
            std::thread::sleep(Duration::from_millis(sleep)); // 16 ~60 FPS
        }
    }

    pub fn lighting(&self, material: &Material, light: &Light, point: &Point, eyev: &Vector, normalv: &Vector) -> Color {
        let effective_color = material.color * light.intensity;
        let lightv = (light.position - *point).normalize();

        let ambient = effective_color * material.ambient;
        let light_dot_normal = lightv.dot(normalv);

        let diffuse;
        let specular;
        if light_dot_normal < 0. {
            diffuse = Color::new([0., 0., 0.]);
            specular = Color::new([0., 0., 0.]);
        }
        else {
            diffuse = effective_color * material.diffuse * light_dot_normal;
            let reflectv = (lightv * -1.).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0. {
                specular = Color::new([0., 0., 0.]);
            }
            else {
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = light.intensity * material.specular * factor;
            }
            
        }
        ambient + diffuse + specular
    }

    pub fn lighting_ext(material: &Material, light: &Light, point: &Point, eyev: &Vector, normalv: &Vector) -> Color {
        let effective_color = material.color * light.intensity;
        let lightv = (light.position - *point).normalize();

        let ambient = effective_color * material.ambient;
        let light_dot_normal = lightv.dot(normalv);

        let diffuse;
        let specular;
        if light_dot_normal < 0. {
            diffuse = Color::new([0., 0., 0.]);
            specular = Color::new([0., 0., 0.]);
        }
        else {
            diffuse = effective_color * material.diffuse * light_dot_normal;
            let reflectv = (lightv * -1.).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0. {
                specular = Color::new([0., 0., 0.]);
            }
            else {
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = light.intensity * material.specular * factor;
            }
            
        }
        ambient + diffuse + specular
    }

}


pub fn color_to_u32(color: &Color) -> u32 {
    let r = (color.data.x.clamp(0.0, 1.0) * 255.0) as u32;
    let g = (color.data.y.clamp(0.0, 1.0) * 255.0) as u32;
    let b = (color.data.z.clamp(0.0, 1.0) * 255.0) as u32;

    (r << 16) | (g << 8) | b
}

