/////////////////////////////////////////////////////////////////////////////////////
//
// ! Ajouter un option anti-alyasing, modifier la fonction update_image
//
// ! Changer la fonction update_image pour qu'elle soit multithreade
//
/////////////////////////////////////////////////////////////////////////////////////

// use minifb::{Key, Window};
use minifb::{Key, Window, WindowOptions};

use std::thread;

// use crate::{Camera, Canvas, Direction, World};
use crate::{Camera, Canvas, World, Point, Vector, Color, Matrix, Ray, Direction, Intersection, Light};

pub struct Renderer {
    pub window: Window,
    pub canvas: Canvas,
    pub world: World,
    pub camera: Camera,
    pub sky: Color,
    // pub size: (usize, usize),
    // enlever size pose pb avec la fonction render du projet d'origine

}

impl Renderer {
    pub fn new(canvas: Canvas, world: World) -> Result<Self, minifb::Error> { // return an error in case of window error
        match Window::new(
            "RT",
            canvas.width,
            canvas.height,
            WindowOptions {
                resize: true,
                ..WindowOptions::default()
            }
        ) {
            Ok(window) => {
                let camera = Camera::new(
                    Point::new(0., 0., 10.),
                    Vector::new(0., 0., -1.),
                    canvas.width as f64 / canvas.height as f64,
                    45f64.to_radians(),
                    0.1,
                    100.,
                );
                let mut new_world = world.clone();
                for object in &mut new_world.objects {
                    object.update();
                    object.get_texture();
                }
                Ok(Self {
                    window,
                    canvas,
                    world: new_world,
                    camera,
                    sky: Color::new(0., 0., 0.), // a basculer dans le monde ?
                })
            }
            Err(e) => {
                Err(e)
            }
        }
    }

// shadow and light

    pub fn is_shadowed(&self, point: &Point, light: &Light) -> bool {
        let v = light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(*point, direction);

        let intersection = self.world.intersect(&r, 1.);

        if intersection.is_some() {
            let h = intersection.unwrap();
            if h.t < distance {
                return true;
            }
        }

        false
    }

    pub fn shade_it(&self, comps: &Intersection) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        let light_number = self.world.lights.len();

        for light in &self.world.lights {
            let shadowed = self.is_shadowed(&comps.over_point, light);

            let temporary_color = World::lighting(
                &comps.object,
                light,
                &comps.over_point,
                &comps.hit_normal,
                &comps.normal,
                shadowed,
            );

            color += temporary_color * (1.0 / light_number as f64);
        }

        color
    }

    pub fn get_phong_color(&self, initial_hit: &Intersection, ) -> Color {
        let mut reflected_color = Color::new(0.0, 0.0, 0.0);
        let mut first_hit = initial_hit.clone();
        let mut factor = 1.0;

        // < 1.

        if first_hit.object.material.reflective > 0. {
            for _ in 0..1 {
                let reflected_ray = Ray::new(first_hit.point, first_hit.reflectv);

                let reflected_hit = self.world.intersect(&reflected_ray, 1.);
                if reflected_hit.is_some() {
                    let reflected_inter = reflected_hit.unwrap();
                    reflected_color += self.shade_it(&reflected_inter)
                        * first_hit.object.material.reflective
                        * factor;
                    factor = factor * 0.20;
                    first_hit = reflected_inter;
                } else {
                    break;
                }
            }
        }
        
        let color = self.shade_it(&initial_hit) + reflected_color;
        color
    }

    pub fn get_pixel(&mut self, ray: &Ray) -> Color {
        let hit: Option<Intersection> = self.world.intersect(ray, 1.);
        match hit {
            Some(inter) => {
                // eprintln!("Je tombe sur l'obj {:?} pos: {:?}", inter.object.material.color, inter.object.position);
                // inter.object.material.color//+ Color::new(0.1, 0.1, 0.1)
                self.get_phong_color(&inter) + Color::new(0.1, 0.1, 0.1)
            }
            None => {

                self.sky
            }
        }
    }

    pub fn update_image(&mut self) {

        let view = Matrix::view(
            self.camera.position,
            self.camera.position + self.camera.direction(),
            Vector::new(0., 1., 0.),
        );

        let projection = Matrix::projection(self.camera.fov, self.camera.aspect, self.camera.near, self.camera.far);

        let view_proj = projection * view;
        let inv_view_proj = view_proj.inverse().unwrap();

        for y in 0..self.canvas.height {
            let ndc_y = 1.0 - 2.0 * ((self.canvas.height - y) as f64 + 0.5) / self.canvas.height as f64;

            for x in 0..self.canvas.width {
                let ndc_x = 2.0 * (x as f64 + 0.5) / self.canvas.width as f64 - 1.0;

                let origin = inv_view_proj.clone() * Point::new(ndc_x, ndc_y, -1.0);
                let target = inv_view_proj.clone() * Point::new(ndc_x, ndc_y, 1.0);

                let direction = (target - origin).normalize();

                let ray = Ray::new(Point::new(origin.x, origin.y, origin.z), direction);
                
                // lancer un boucle lancer plusieurs rayons pour le meme pixel


                // faire une variation du ray

                // faire la moyenne entre les differents rayons

                let color = self.get_pixel(&ray);
                self.canvas.write(x as usize, y as usize, color);
                
            }
        }
    }

    pub fn render(&mut self) -> Result<u32, minifb::Error> {

        let mut handles = Vec::new();
    
        // Créer plusieurs threads et stocker leurs handles
        for i in 0..5 {
            let handle = thread::spawn(move || {
                println!("Thread {} en cours d'exécution", i);
                // Votre logique ici
                i * 2 // retourner une valeur
            });
            handles.push(handle);
        }

        while self.window.is_open() {
            if self.window.is_key_down(Key::Escape) {
                break;
            }
            let current_size = self.window.get_size();
            let size = (self.canvas.width, self.canvas.height);

            if size != current_size {
                self.canvas.resize(current_size.0, current_size.1);
                self.camera.resize(size.0 as f64 / size.1 as f64);
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

            self.update_image();

            let buffer = self.canvas.pixels();
            match self.window
                .update_with_buffer(&buffer, current_size.0, current_size.1) {
                    Ok(_) => {

                    }
                    Err(e) => {
                        eprintln!("Error uptating window: {:?}", e);
                        return Err(e);
                    }
                };
        }
        for handle in handles {
            let result = handle.join().unwrap();
            println!("thread result: {}", result);
        }
        Ok(0)

    }

}
