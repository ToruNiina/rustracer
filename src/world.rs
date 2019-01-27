use crate::sphere::Sphere;
use crate::vector::{Vector3, Vector4};
use crate::background::Background;
use crate::ray::Ray;
use crate::collide::{CollideResult, Collide};
use std::cmp::PartialOrd;

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

    pub fn color<B>(&self, ray: &Ray, background: &B) -> Vector4
    where
        B:Background,
    {
        let hits = self.objects.iter()
            .flat_map(|obj| obj.collide(&ray))
            .collect::<std::vec::Vec<_>>();

        if hits.is_empty() {
            background.color_ratio_at(ray.direction)
        } else {

            let nearest = hits.iter().min_by(
                |lhs, rhs| lhs.t.partial_cmp(&rhs.t).unwrap_or(std::cmp::Ordering::Equal)
                ).expect("all the t's are comparable");

            Vector4::new(nearest.normal[0] * 0.5 + 0.5,
                         nearest.normal[1] * 0.5 + 0.5,
                         nearest.normal[2] * 0.5 + 0.5, 1.0)
        }
    }
}
