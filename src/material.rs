use crate::vector::*;
use crate::collide::CollideResult;
use crate::ray::Ray;

pub trait Scatter {
    /// returns the next ray and the attenuation of the color.
    fn scatter(&self, ray: &Ray, cr: CollideResult, rng: &mut rand::rngs::ThreadRng)
        -> std::option::Option<(Ray, (f32, f32, f32))>;
}

#[derive(Debug)]
pub struct Diffuse {
    albedo: (f32, f32, f32),
}

impl Diffuse {

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Diffuse{albedo: (r, g, b)}
    }

}

impl Scatter for Diffuse {
    fn scatter(&self, ray: &Ray, cr: CollideResult, rng: &mut rand::rngs::ThreadRng)
        -> std::option::Option<(Ray, (f32, f32, f32))> {

        let start = ray.at(cr.t);
        let dir   = cr.normal + pick_in_sphere(&mut *rng);

        Some((Ray::new(start, dir), self.albedo))
    }
}

pub struct Metalic {
    albedo: (f32, f32, f32),
}

impl Metalic {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Metalic{albedo: (r, g, b)}
    }
}

impl Scatter for Metalic {
    fn scatter(&self, ray: &Ray, cr: CollideResult, _rng: &mut rand::rngs::ThreadRng)
        -> std::option::Option<(Ray, (f32, f32, f32))> {

        let start = ray.at(cr.t);
        let reflected = reflect(ray.direction, cr.normal);

        Some((Ray::new(start, reflected), self.albedo))
    }
}

pub enum Material {
    Diffuse(Diffuse),
    Metalic(Metalic),
}

impl Material {
    pub fn make_diffuse(r: f32, g: f32, b: f32) -> Self {
        Material::Diffuse(Diffuse::new(r, g, b))
    }
    pub fn make_metalic(r: f32, g: f32, b: f32) -> Self {
        Material::Metalic(Metalic::new(r, g, b))
    }
}

impl Scatter for Material {
    fn scatter(&self, ray: &Ray, cr: CollideResult, rng: &mut rand::rngs::ThreadRng)
        -> std::option::Option<(Ray, (f32, f32, f32))> {
        match self {
            Material::Diffuse(mt) => {mt.scatter(ray, cr, rng)}
            Material::Metalic(mt) => {mt.scatter(ray, cr, rng)}
        }
    }
}
