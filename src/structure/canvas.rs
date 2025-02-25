use std::fs::File;
use std::io::{self, Write};

use crate::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    // handle negative values ?
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new([0.0, 0.0, 0.0]); width * height],
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.pixels = vec![Color::new([0., 0., 0.]); width * height];
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
        self.pixels
            .iter()
            .map(|c| c.clone().into())
            .collect()
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
}
