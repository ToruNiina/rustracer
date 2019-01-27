use crate::sphere::Sphere;
use crate::vector::{Vector3, Vector4};
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
    fn collide(&self, ray: &Ray) -> Option<CollideResult> {
        match &self {
            Object::Sphere(sph, _) => {
                sph.collide(ray)
            }
        }
    }
}

impl Scatter for Object {
    fn scatter(&self, ray: &Ray, cr: CollideResult, rng: &mut rand::rngs::ThreadRng)
        -> std::option::Option<(Ray, (f32, f32, f32))> {
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

    pub fn pick_in_sphere(rng: &mut rand::rngs::ThreadRng) -> Vector3 {
        let u = rng.gen_range(0.0f32, 1.0f32);
        let normal = rand::distributions::StandardNormal;
        Vector3::unit(Vector3::new(normal.sample(&mut *rng) as f32,
                                   normal.sample(&mut *rng) as f32,
                                   normal.sample(&mut *rng) as f32)) * u.cbrt()
    }

    pub fn color<B>(&self,
                    ray: &Ray,
                    background: &B,
                    rng: &mut rand::rngs::ThreadRng,
                    depth: usize) -> Vector4
    where
        B:Background,
    {
        if depth >= 50 {
            return Vector4::zero();
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
            if let Some((next_ray, (att_r, att_g, att_b))) =
                nearest.scatter(ray, collide, &mut *rng) {
                let next_color = self.color(&next_ray, background, &mut *rng, depth+1);

                Vector4::new(next_color[0] * att_r,
                             next_color[1] * att_g,
                             next_color[2] * att_b,
                             next_color[3])
            } else {
                Vector4::zero()
            } 
        } else {
            background.color_ratio_at(ray.direction)
        }
    }
}
