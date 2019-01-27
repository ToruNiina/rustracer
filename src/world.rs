use crate::sphere::Sphere;
use crate::vector::Vector3;

pub enum Object {
    Sphere(Sphere),
}

impl Object {
    pub fn make_sphere(center: Vector3, radius: f32) -> Object {
        Object::Sphere(Sphere::new(center, radius))
    }
}

impl std::convert::From<Sphere> for Object {
    fn from(sph: Sphere) -> Self {
        Object::Sphere(sph)
    }
}

pub struct World {
    pub objects: std::vec::Vec<Object>,
}

impl World {
    pub fn new(objects: std::vec::Vec<Object>) -> World {
        World{objects}
    }
}
