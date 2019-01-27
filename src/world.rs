use crate::sphere::Sphere;
use crate::vector::{Vector3, Vector4};
use crate::ray::Ray;
use crate::collide::{CollideResult, Collide};

pub enum Object {
    Sphere(Sphere),
}

impl Object {
    pub fn make_sphere(center: Vector3, radius: f32) -> Object {
        Object::Sphere(Sphere::new(center, radius))
    }
}

impl Collide for Object {
    fn collide(&self, ray: &Ray) -> Option<CollideResult> {
        match &self {
            Object::Sphere(sph) => {
                sph.collide(ray)
            }
        }
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

    pub fn color(&self, ray: Ray) -> Vector4 {
        let mut color = Vector4::zero();

        let mut min_t = std::f32::INFINITY;
        for obj in self.objects.iter() {
            if let Some(CollideResult{t, normal}) = obj.collide(&ray) {
                if t < min_t {
                    min_t = t;
                    color = Vector4::new(normal[0] * 0.5 + 0.5,
                                         normal[1] * 0.5 + 0.5,
                                         normal[2] * 0.5 + 0.5, 1.0);
                }
            }
        }
        color
    }
}
