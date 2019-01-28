use crate::vector::*;
use crate::color::RGB;
use crate::collide::Collision;
use crate::ray::Ray;
use crate::util::*;
use rand::Rng;

pub trait Scatter {
    /// returns the next ray and the attenuation of the color.
    fn scatter<R: Rng>(&self, ray: &Ray, cr: Collision, rng: &mut R) -> (Ray, RGB);
}

#[derive(Debug)]
pub struct Diffuse {
    albedo:   RGB,
}

impl Diffuse {
    pub fn new(albedo: RGB) -> Self {
        Diffuse{albedo}
    }
}

impl Scatter for Diffuse {
    fn scatter<R: Rng>(&self, ray: &Ray, cr: Collision, rng: &mut R)
        -> (Ray, RGB) {

        let start = ray.at(cr.t);
        let dir   = cr.normal + pick_in_sphere(&mut *rng);
        (Ray::new(start, dir), self.albedo)
    }
}

#[derive(Debug)]
pub struct Metalic {
    fuzziness: f32,
    albedo: RGB,
}

impl Metalic {
    pub fn new(fuzziness: f32, albedo: RGB) -> Self {
        Metalic{fuzziness: clamp(fuzziness, 0.0, 1.0), albedo}
    }
}

impl Scatter for Metalic {
    fn scatter<R: Rng>(&self, ray: &Ray, cr: Collision, rng: &mut R)
        -> (Ray, RGB) {

        let start = ray.at(cr.t);
        let reflected = if self.fuzziness == 0.0 {
            reflect(ray.direction, cr.normal)
        } else {
            reflect(ray.direction, cr.normal) +
                self.fuzziness * pick_in_sphere(&mut *rng)
        };

        (Ray::new(start, reflected), self.albedo)
    }
}

#[derive(Debug)]
pub struct Dielectric {
    refidx: f32,
    albedo: RGB,
}

impl Dielectric {
    pub fn new(refidx: f32, albedo: RGB) -> Self {
        Dielectric{refidx, albedo}
    }

    pub fn schlick(&self, cosine: f32) -> f32 {
        let r0 = (1.0 - self.refidx) / (1.0 + self.refidx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter<R: Rng>(&self, ray: &Ray, cr: Collision, rng: &mut R)
        -> (Ray, RGB) {

        let start = ray.at(cr.t);

        let (out_normal, ni_over_nt, cosine) =
            if Vector3::dot(ray.direction, cr.normal) > 0.0 {
                (-cr.normal,       self.refidx,  ray.direction.dot(cr.normal))
            } else {
                ( cr.normal, 1.0 / self.refidx, -ray.direction.dot(cr.normal))
            };

        let reflected = reflect(ray.direction, cr.normal);
        if let Some(refracted) = refract(ray.direction, out_normal, ni_over_nt) {
            if rng.gen_range(0.0f32, 1.0f32) < self.schlick(cosine) {
                (Ray::new(start, reflected), self.albedo)
            } else {
                (Ray::new(start, refracted), self.albedo)
            }
        } else {
            (Ray::new(start, reflected), self.albedo)
        }
    }
}

pub enum Material {
    Diffuse   {diffuse:    Diffuse   },
    Metalic   {metalic:    Metalic   },
    Dielectric{dielectric: Dielectric},
}

impl Material {
    pub fn make_diffuse(rgb: RGB) -> Self {
        Material::Diffuse{
            diffuse: Diffuse::new(rgb),
        }
    }
    pub fn make_metalic(f: f32, rgb: RGB) -> Self {
        Material::Metalic{
            metalic: Metalic::new(f, rgb),
        }
    }
    pub fn make_dielectric(n: f32, rgb: RGB) -> Self {
        Material::Dielectric{
            dielectric: Dielectric::new(n, rgb),
        }
    }
}

impl Scatter for Material {
    fn scatter<R: Rng>(&self, ray: &Ray, cr: Collision, rng: &mut R)
        -> (Ray, RGB) {
        match self {
            Material::Diffuse{diffuse: mt}       => {mt.scatter(ray, cr, rng)}
            Material::Metalic{metalic: mt}       => {mt.scatter(ray, cr, rng)}
            Material::Dielectric{dielectric: mt} => {mt.scatter(ray, cr, rng)}
        }
    }
}
