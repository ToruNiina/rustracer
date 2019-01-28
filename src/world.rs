use crate::sphere::Sphere;
use crate::vector::Vector3;
use crate::color::RGB;
use crate::background::Background;
use crate::ray::Ray;
use crate::collide::{Collision, Collide};
use crate::material::{Scatter, Material};
use rand::Rng;

pub enum Shape {
    Sphere(Sphere),
}

pub struct Object {
    shape:    Shape,
    albedo:   RGB,
    material: Material,
}

impl Object {
    pub fn make_sphere(sphere: Sphere, material: Material, albedo: RGB) -> Object {
        Object{shape: Shape::Sphere(sphere), albedo, material}
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

pub struct World<Bg> {
    pub objects: std::vec::Vec<Object>,
        bg:      Bg,
}

impl<Bg: Background> World<Bg> {
    pub fn new(objects: std::vec::Vec<Object>, bg: Bg) -> World<Bg> {
        World{objects, bg}
    }

    /// returns color & depth of the recursion.
    pub fn color<R>(&self, ray: Ray, rng: &mut R, depth: usize) -> (RGB, usize)
    where
        R: Rng
    {
        const DEPTH_LIMIT: usize = 100;
        if depth >= DEPTH_LIMIT {
            return (RGB::new(0.0, 0.0, 0.0), DEPTH_LIMIT);
        }

        let mut nearest = None;
        let mut min_t   = std::f32::INFINITY;
        for obj in self.objects.iter() {
            if let Some(collide) =
                obj.collide_within(&ray, 0.0001, std::f32::INFINITY) {

                if collide.t < min_t {
                    min_t = collide.t;
                    nearest = Some((obj, collide))
                }
            }
        }

        if let Some((nearest, collide)) = nearest {
            let next_ray = nearest.scatter(&ray, collide, rng);
            let (c, d)   = self.color(next_ray, rng, depth+1);
            (nearest.albedo * c, d)
        } else {
            (self.bg.color_at(ray.direction), depth)
        }
    }
}
