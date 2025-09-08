use crate::EPSILON;
use crate::{Intersection, Material, Matrix, Point, Ray, Shape, Vector, Texture, InfoTexture, TextureType};

use image::open;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    pub material: Material,
    pub position: Point,
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
    pub scale: Vector,
    pub shape: Shape,

    pub world_to_local: Matrix,
    pub local_to_world: Matrix,

    pub tex_img_name: Option<Vec<InfoTexture>>,

}

impl Object {
    pub fn new(shape: Shape) -> Object {
        Object {
            material: Material::new(),
            position: Point::new(0., 0., 0.),
            pitch: 0.,
            yaw: 0.,
            roll: 0.,
            shape,
            scale: Vector::new(1., 1., 1.),
            world_to_local: Matrix::identity(),
            local_to_world: Matrix::identity(),
            tex_img_name: None,
        }
    }

    pub fn material(mut self, material: Material) -> Self {
        self.material = material;

        self
    }

    pub fn update(&mut self) {
        let vt = Vector::new(self.position.x, self.position.y, self.position.z);

        let translation = Matrix::translation(vt);
        let rotation = Matrix::rotation(self.pitch, self.yaw, self.roll);
        let scaling = Matrix::scaling(self.scale.clone());

        self.local_to_world = translation * rotation * scaling;
        self.world_to_local = self.local_to_world.inverse().unwrap();
    }

    pub fn get_texture(&mut self) {
        if self.tex_img_name.is_some() {
            // eprintln!("I have found a vector of texture names");
            let mut textures = Vec::new();
            for img in self.tex_img_name.clone().unwrap() {
                match open(img.clone().filename) {
                    Ok(ok) => {
                        // eprintln!("I opened a texture {}", img.filename);
                        match img.kind.as_str() {
                            "Diffuse" => {
                                textures.push(Texture::new(ok, TextureType::Diffuse));
                            }
                            "Normal" => {
                                textures.push(Texture::new(ok, TextureType::Normal));
                            }
                            "Specular" => {
                                textures.push(Texture::new(ok, TextureType::Specular));
                            }
                            "Detail" => {
                                textures.push(Texture::new(ok, TextureType::Detail));
                            }
                            "Emission" => {
                                textures.push(Texture::new(ok, TextureType::Emission));
                            }
                            _ => {
                                textures.push(Texture::new(ok, TextureType::Diffuse));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error when opening a file ({}): {}", img.filename, e);
                    }
                }

            }
            self.material.tex = Some(textures);
        }
    }

    pub fn intersect(&self, ray: &Ray, n1: f64) -> Option<Intersection> {
        // Transform ray to local space
        let local_ray = self.world_to_local.clone() * ray.clone();

        // Delegate to shape's local-space intersection logic
        if let Some(local_hit) = self.shape.intersect(local_ray) {
            // Transform hit data back to WORLD space
            let world_point: Point = self.local_to_world.clone() * local_hit.point;
            let world_normal: Vector = (self.local_to_world.clone() * local_hit.normal).normalize();

            let over_point = world_point + world_normal * EPSILON;

            Some(Intersection::new(
                self,
                local_hit.t,
                world_point,
                world_normal,
                -(ray.direction),
                over_point,
                (ray.direction).reflect(&world_normal),
                n1,
            ))
        } else {
            None
        }
    }

}

// this trait will be associate to Patterns, so maybe he deserve it own file ?
pub trait Transform {
    fn rotate(&mut self, pitch: f64, yaw: f64, roll: f64);
    fn translate(&mut self, vec: Vector);
    fn scale(&mut self, vec: Vector);
}

impl Transform for Object {
    // move obj
    fn translate(&mut self, vec: Vector) {
        self.position = self.position.clone() + vec;

        self.update();
    }

    fn rotate(&mut self, pitch: f64, yaw: f64, roll: f64) {
        self.pitch = pitch;
        self.yaw = yaw;
        self.roll = roll;

        self.update();
    }

    // grow, shrink the object
    fn scale(&mut self, vec: Vector) {
        self.scale = vec;

        self.update();
    }
}
