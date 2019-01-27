use crate::sphere::Sphere;
use crate::vector::{Vector3, Vector4};
use crate::background::Background;
use crate::ray::Ray;
use crate::collide::{CollideResult, Collide};
use rand::distributions::Distribution;
use rand::Rng;

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

    pub fn pick_in_sphere(rng: &mut rand::rngs::ThreadRng) -> Vector3 {
        let u = &mut rng.gen_range(0.0f32, 1.0f32);
        let normal = rand::distributions::StandardNormal;
        Vector3::unit(Vector3::new(normal.sample(&mut *rng) as f32,
                                   normal.sample(&mut *rng) as f32,
                                   normal.sample(&mut *rng) as f32)) * u.cbrt()
    }

    pub fn color<B>(&self,
                    ray: &Ray,
                    background: &B,
                    rng: &mut rand::rngs::ThreadRng) -> Vector4
    where
        B:Background,
    {
        let hits = self.objects.iter()
            .flat_map(|obj| obj.collide_within(&ray, 0.0001, std::f32::INFINITY))
            .collect::<std::vec::Vec<_>>();

        if hits.is_empty() {
            background.color_ratio_at(ray.direction)
        } else {

            let nearest = hits.iter().min_by(
                |lhs, rhs| lhs.t.partial_cmp(&rhs.t).unwrap_or(std::cmp::Ordering::Equal)
                ).expect("all the t's are comparable");

            let start = ray.at(nearest.t);
            let dir   = nearest.normal + Self::pick_in_sphere(&mut *rng);

            0.5 * self.color(&Ray::new(start, dir), background, &mut *rng)
        }
    }
}
