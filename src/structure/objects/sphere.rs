struct Sphere {
    center: Point,
    radius: f32,
    shared_vars: my_vars, 
}



impl Sphere {



}

impl Object for Sphere {

    fn woami(&self) -> u32;

}

struct Obj<T> {
    shape: T,
    position: Point,

}