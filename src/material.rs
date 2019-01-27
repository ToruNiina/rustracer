use crate::vector::{Vector3, Vector4};
use crate::collide::CollideResult;
use crate::ray::Ray;
use rand::distributions::Distribution;
use rand::Rng;

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

    pub fn pick_in_sphere(rng: &mut rand::rngs::ThreadRng) -> Vector3 {
        let u = rng.gen_range(0.0f32, 1.0f32);
        let normal = rand::distributions::StandardNormal;
        Vector3::unit(Vector3::new(normal.sample(&mut *rng) as f32,
                                   normal.sample(&mut *rng) as f32,
                                   normal.sample(&mut *rng) as f32)) * u.cbrt()
    }
}

impl Scatter for Diffuse {
    fn scatter(&self, ray: &Ray, cr: CollideResult, rng: &mut rand::rngs::ThreadRng)
        -> std::option::Option<(Ray, (f32, f32, f32))> {

        let start = ray.at(cr.t);
        let dir   = cr.normal + Self::pick_in_sphere(&mut *rng);

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

    pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
        v - 2.0 * Vector3::dot(v, n) * n
    }
}

impl Scatter for Metalic {
    fn scatter(&self, ray: &Ray, cr: CollideResult, _rng: &mut rand::rngs::ThreadRng)
        -> std::option::Option<(Ray, (f32, f32, f32))> {

        let start = ray.at(cr.t);
        let reflected = Self::reflect(ray.direction, cr.normal);

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
