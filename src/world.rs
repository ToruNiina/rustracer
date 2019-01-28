use crate::sphere::Sphere;
use crate::vector::Vector3;
use crate::color::{Color, RGBA, RGB};
use crate::background::Background;
use crate::ray::Ray;
use crate::collide::{CollideResult, Collide};
use crate::material::{Scatter, Material};
use rand::distributions::Distribution;
use rand::Rng;

use std::cmp::PartialOrd;

pub enum Object {
    Sphere(Sphere, Material),
}

impl Object {
    pub fn make_sphere(center: Vector3, radius: f32, mat: Material) -> Object {
        Object::Sphere(Sphere::new(center, radius), mat)
    }
}

impl Collide for Object {
    fn collide_within(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<CollideResult> {
        match &self {
            Object::Sphere(sph, _) => {
                sph.collide_within(ray, t_min, t_max)
            }
        }
    }
}

impl Scatter for Object {
    fn scatter(&self, ray: &Ray, cr: CollideResult, rng: &mut rand::rngs::ThreadRng)
        -> std::option::Option<(Ray, RGB)> {
        match &self {
            Object::Sphere(_, mat) => {
                mat.scatter(ray, cr, rng)
            }
        }
    }
}

pub struct World {
    pub objects: std::vec::Vec<Object>,
}

impl World {
    pub fn new(objects: std::vec::Vec<Object>) -> World {
        World{objects}
    }

    pub fn color<B>(&self,
                    ray: &Ray,
                    background: &B,
                    rng: &mut rand::rngs::ThreadRng,
                    depth: usize) -> RGB
    where
        B:Background,
    {
        if depth >= 100 {
            return RGB::new(0.0, 0.0, 0.0)
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
            if let Some((next_ray, att)) = nearest.scatter(ray, collide, rng) {
                att * self.color(&next_ray, background, &mut *rng, depth+1)
            } else {
                RGB::new(0.0, 0.0, 0.0)
            }
        } else {
            From::from(background.color_at(ray.direction))
        }
    }
}
