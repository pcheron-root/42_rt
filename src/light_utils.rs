use std::f32::EPSILON;

use crate::{Color, Intersect, Intersection, Point, Ray, World};

pub fn is_shadowed(world: &World, point: &Point) -> bool {
    let v = world.light.position - *point;
    let distance = v.magnitude();
    let direction = v.normalize();

    let r = Ray::new(*point, direction);

    let intersection = world.intersect(r);

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

pub fn get_phong_color(world: &World, initial_hit: Intersection) -> Color {
    let mut reflected_color= Color::new(0.0, 0.0, 0.0);
    let mut first_hit = initial_hit.clone();
    let mut factor = 1.0;
    if first_hit.object.material.reflective > 0. {
        for _ in 0..1 {
            let reflected_ray = Ray::new(first_hit.point, first_hit.reflectv);

            let reflected_hit = world.intersect(reflected_ray);
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
    let color = shade_it(&world, &initial_hit) + reflected_color;
    color
}

// refraction :

pub fn get_over_point(inter: &Intersection) -> Point {
    let surface = -inter.normal * EPSILON * 2.;
    let point = inter.point.clone() + surface;
    point
}

// pub fn get_color_refracted(inter: &Intersection)
