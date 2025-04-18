use crate::{Color, Intersection, Point, Ray, World};

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
