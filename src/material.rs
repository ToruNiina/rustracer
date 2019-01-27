use crate::vector::*;
use crate::collide::CollideResult;
use crate::ray::Ray;
use crate::util::*;

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
    fuzziness: f32,
    albedo: (f32, f32, f32),
}

impl Metalic {
    pub fn new(fuzziness: f32, r: f32, g: f32, b: f32) -> Self {
        Metalic{fuzziness: clamp(fuzziness, 0.0, 1.0), albedo: (r, g, b)}
    }
}

impl Scatter for Metalic {
    fn scatter(&self, ray: &Ray, cr: CollideResult, rng: &mut rand::rngs::ThreadRng)
        -> std::option::Option<(Ray, (f32, f32, f32))> {

        let start = ray.at(cr.t);
        let reflected = if self.fuzziness == 0.0 {
            Vector3::unit(reflect(ray.direction, cr.normal))
        } else {
            Vector3::unit(reflect(ray.direction, cr.normal)) +
                self.fuzziness * pick_in_sphere(&mut *rng)
        };

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
    pub fn make_metalic(f: f32, r: f32, g: f32, b: f32) -> Self {
        Material::Metalic(Metalic::new(f, r, g, b))
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
