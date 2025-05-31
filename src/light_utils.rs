use std::f32::EPSILON;
use log::{info, error};
use crate::{Color, Intersection, Point, Ray, World};

pub fn is_shadowed(world: &World, point: &Point) -> bool {
    let v = world.light.position - *point;
    let distance = v.magnitude();
    let direction = v.normalize();

    let r = Ray::new(*point, direction);

    let intersection = world.intersect(&r, 1., 2);

    if intersection.is_some() {
        let h = intersection.unwrap();
        if h.t < distance {
            return true;
        }
    }

    false
}

pub fn shade_it(world: &World, comps: &Intersection) -> Color {
    let shadowed = is_shadowed(world, &comps.over_point);

    World::lighting(
        &comps.object,
        &world.light,
        &comps.over_point,
        &comps.hit_normal,
        &comps.normal,
        shadowed,
    )
}

pub fn get_color_from_ray(world: &World, ray: &Ray, sky: &Color, n1: f32, remaning: u32) -> Color {
    let hit = world.intersect(ray, 1., remaning);
    if hit.is_some() {
        let color = get_phong_color(&world, hit.unwrap());
        return color;
    }
    sky.clone()
}

pub fn get_phong_color(world: &World, initial_hit: Intersection) -> Color {
    let mut reflected_color= Color::new(0.0, 0.0, 0.0);
    let mut first_hit = initial_hit.clone();
    let mut factor = 1.0;
    if first_hit.object.material.reflective > 0. {
        for _ in 0..1 {
            let reflected_ray = Ray::new(first_hit.point, first_hit.reflectv);

            let reflected_hit = world.intersect(&reflected_ray, 1., 2);
            if reflected_hit.is_some() {
                let reflected_inter = reflected_hit.unwrap();
                reflected_color += shade_it(&world, &reflected_inter) * first_hit.object.material.reflective * factor;
                factor = factor * 0.20;
                first_hit = reflected_inter;
            }
            else {
                break;
            }
            
        }
    }
    //
    let color = shade_it(&world, &initial_hit) + reflected_color;
    color
}

// refraction :

pub fn get_over_point(inter: &Intersection) -> Point {
    let surface = -inter.normal * EPSILON * 2.;
    let point = inter.point.clone() + surface;
    point
}

pub fn get_refracted_color(world: &World, inter: &Intersection, remaining: u32) -> Color {

    if remaining == 0 || inter.object.material.transparency == 0. {
        println!("wtf je suis pas transparent");
        return Color::new(0., 0., 0.);
    };
    let eta = inter.n1 / inter.object.material.refractive_index;
    let cos_i = (-inter.hit_normal).dot(&inter.normal);
    let sin2_t = eta * eta * (1.0 - cos_i * cos_i);
    
    if sin2_t > 1.0 {
        println!("wtf je reflete l'interieur");
        return Color::new(0., 0., 0.); // Réflexion totale interne, pas de rayon réfracté
    }
    
    let cos_t = (1.0 - sin2_t).sqrt();
    let refracted = (-inter.hit_normal * eta + inter.normal * (eta * cos_i - cos_t)).normalize();
    // Some(refracted.normalize());
    // direction ← comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio
    let over_point = get_over_point(inter);
    let ray = Ray { origin: over_point, direction: refracted };



    get_color_from_ray(world, &ray, &Color::new(0., 0., 0.), inter.object.material.refractive_index, remaining - 1)
}

// pub fn get_color_refracted(inter: &Intersection)
