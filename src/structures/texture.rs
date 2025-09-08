
use crate::{Color, Point, Object};

use image::{DynamicImage, Rgba, GenericImageView};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InfoTexture {
    pub filename: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextureType {
    Diffuse,  // Texture principale/couleur de base -> ok
    Specular, // Map de réflectivité -> ok
    Emission, // Texture pour zones non éclairées -> ok
    Detail,   // Texture secondaire/détails
    Normal,   // Bump/normal mapping
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub image: DynamicImage,
    pub width: u32,
    pub height: u32,
    pub kind: TextureType,
}

impl Texture {
    pub fn new(image: DynamicImage, kind: TextureType) -> Self {
        let width = image.width();
        let height = image.height();
        Self {
            image,
            width,
            height,
            kind,
        }
    }

    pub fn stripe_at_object(&self, obj: &Object, world_point: &Point) -> Color {
        let obj_point = obj.world_to_local.clone() * *world_point;
        // let pattern_point = self.world_to_local.clone() * obj_point;

        let coord = TextureMapping::spherical_uv(&obj_point);
        self.sample_uv(coord.0, coord.1)
    }

    pub fn sample_uv(&self, u: f64, v: f64) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let x = (((1. - u) * self.width as f64) as u32).min(self.width - 1);
        let y = ((v * self.height as f64) as u32).min(self.height - 1);

        let pixel = self.image.get_pixel(x, y);
        
        Color {
            r: pixel[0] as f64 / 255.0,
            g: pixel[1] as f64 / 255.0,
            b: pixel[2] as f64 / 255.0,
        }
    }

    // unused
    // Échantillonner avec filtrage bilinéaire pour une meilleure qualité
    pub fn sample_uv_filtered(&self, u: f64, v: f64) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        // Coordonnées continues
        let x_float = u * (self.width - 1) as f64;
        let y_float = (1.0 - v) * (self.height - 1) as f64;

        // Coordonnées des 4 pixels voisins
        let x1 = x_float.floor() as u32;
        let y1 = y_float.floor() as u32;
        let x2 = (x1 + 1).min(self.width - 1);
        let y2 = (y1 + 1).min(self.height - 1);

        // Facteurs d'interpolation
        let fx = x_float - x1 as f64;
        let fy = y_float - y1 as f64;

        // Échantillonner les 4 pixels
        let c00 = self.pixel_to_color(self.image.get_pixel(x1, y1));
        let c10 = self.pixel_to_color(self.image.get_pixel(x2, y1));
        let c01 = self.pixel_to_color(self.image.get_pixel(x1, y2));
        let c11 = self.pixel_to_color(self.image.get_pixel(x2, y2));

        // Interpolation bilinéaire
        let c0 = c00 * (1.0 - fx) + c10 * fx;
        let c1 = c01 * (1.0 - fx) + c11 * fx;
        c0 * (1.0 - fy) + c1 * fy
    }

    fn pixel_to_color(&self, pixel: Rgba<u8>) -> Color {
        Color {
            r: pixel[0] as f64 / 255.0,
            g: pixel[1] as f64 / 255.0,
            b: pixel[2] as f64 / 255.0,
        }
    }

}

pub struct TextureMapping;

impl TextureMapping {

    // sphere
    pub fn spherical_uv(point: &Point) -> (f64, f64) {
        // Convert to spherical coordinates
        let theta = point.z.atan2(point.x);
        let radius = (point.x * point.x + point.y * point.y + point.z * point.z).sqrt();
        let phi = (point.y / radius).acos();

        // Convert to UV coordinates
        let u = (theta + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        let v = phi / std::f64::consts::PI;

        (u, v)
    }

    // plane
    pub fn planar_uv(point: &Point) -> (f64, f64) {
        let u = (point.x - point.x.floor()).abs();
        let v = (point.z - point.z.floor()).abs();
        (u, v)
    }

    pub fn cylindrical_uv(point: &Point) -> (f64, f64) {
        let theta = point.z.atan2(point.x);
        let u = (theta + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        let v = (point.y - point.y.floor()).abs();
        (u, v)
    }

    // thorus
    pub fn toroidal_uv(point: &Point, major_radius: f64, minor_radius: f64) -> (f64, f64) {
        // Calculate distance from Y-axis (distance to torus center axis)
        let distance_from_axis = (point.x * point.x + point.z * point.z).sqrt();
        
        // Theta: angle around the major circumference (around Y-axis)
        let theta = point.z.atan2(point.x);
        
        // Find the closest point on the major circumference
        let major_circle_x = major_radius * (theta.cos());
        let major_circle_z = major_radius * (theta.sin());
        
        // Vector from major circle to the point
        let to_point_x = point.x - major_circle_x;
        let to_point_z = point.z - major_circle_z;
        let to_point_y = point.y;
        
        // Phi: angle around the minor circumference (around the tube)
        let phi = to_point_y.atan2((to_point_x * to_point_x + to_point_z * to_point_z).sqrt());
        
        // Convert to UV coordinates [0, 1]
        let u = (theta + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        let v = (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        
        (u, v)
    }
}