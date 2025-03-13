use std::fs::File;
use std::io::{self, Write};

use crate::Color;
use crate::Light;
use crate::Material;
use crate::Point;
use crate::Vector;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        if width <= 0 || height <= 0 {
            panic!("Cannot create a canvas with zero or negative size");
        }

        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.pixels = vec![Color::new(0.0, 0.0, 0.0); width * height];
    }

    pub fn write(&mut self, x: usize, y: usize, color: Color) {
        *self.at_mut(x, y) = color;
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut Color {
        if x < self.width && y < self.height {
            let index = y * self.width + x;

            &mut self.pixels[index]
        } else {
            panic!("");
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Color {
        if x < self.width && y < self.height {
            let index = y * self.width + x;

            self.pixels[index].clone()
        } else {
            panic!("");
        }
    }

    pub fn pixels(&self) -> Vec<u32> {
        self.pixels.iter().map(|c| c.clone().into()).collect()
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = &self.at(x, y);

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


    pub fn lighting(
        &self,
        material: &Material,
        light: &Light,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        shadowed: bool,
    ) -> Color {
        let effective_color = material.color * light.intensity;
        let lightv = (light.position - *point).normalize();

        let ambient = effective_color * material.ambient;
        let light_dot_normal = lightv.dot(normalv);

        if light_dot_normal < 0. {
            return ambient;
        }

        let specular;
        let diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);

        if reflect_dot_eye <= 0. {
            specular = Color::new(0., 0., 0.);
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }

        ambient + diffuse + specular
    }


    pub fn lighting_ext(
        material: &Material,
        light: &Light,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        shadowed: bool,
    ) -> Color {
        let effective_color = material.color * light.intensity;
        let lightv = (light.position - *point).normalize();

        let ambient = effective_color * material.ambient;
        let light_dot_normal = lightv.dot(normalv);


        let diffuse;
        let specular;
        if light_dot_normal < 0. || shadowed == true {
            diffuse = Color::new([0., 0., 0.]);
            specular = Color::new([0., 0., 0.]);

        }

        let specular;
        let diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);

        if reflect_dot_eye <= 0. {
            specular = Color::new(0., 0., 0.);
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }

        ambient + diffuse + specular
    }
}
