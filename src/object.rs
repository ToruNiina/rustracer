use crate::vector::Vector3;
use crate::color::RGB;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::collide::{Collision, Collide};
use crate::material::{Scatter, Material};
use rand::Rng;

pub enum Shape {
    Sphere(Sphere),
}

pub struct Object {
    pub shape:    Shape,
    pub albedo:   RGB,
    pub emission: RGB,
    pub material: Material,
}

impl Object {
    pub fn make_sphere(sphere: Sphere, material: Material, albedo: RGB, emission: RGB) -> Object {
        Object{shape: Shape::Sphere(sphere), albedo, emission, material}
    }
}

impl Collide for Object {
    fn collide_within(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Collision> {
        match &self.shape {
            Shape::Sphere(sphere) => {sphere.collide_within(ray, t_min, t_max)}
        }
    }
}

impl Scatter for Object {
    fn scatter<R: Rng>(&self, ray: &Ray, cr: Collision, rng: &mut R) -> Ray {
        self.material.scatter(ray, cr, rng)
    }
}


