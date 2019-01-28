use crate::vector::Vector3;
use crate::ray::Ray;
use crate::collide::{CollideResult, Collide};

pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Sphere {
        Sphere{center, radius}
    }
}

impl Collide for Sphere {
    fn collide(&self, ray: &Ray) -> Option<CollideResult> {
        let oc = ray.origin - self.center;
        let a  = ray.direction.len_sq();
        let b  = 2.0 * Vector3::dot(oc, ray.direction);
        let c  = oc.len_sq() - self.radius * self.radius;
        let d  = b * b - 4.0 * a * c;

        if d < 0.0 {return None;}

        let sqrt_d = d.sqrt();
        let t = if      -b - sqrt_d > 0.0 {(-b - sqrt_d) / (2.0 * a)}
                else if -b + sqrt_d > 0.0 {(-b + sqrt_d) / (2.0 * a)}
                else {return None};
        let normal = (ray.at(t) - self.center) / self.radius;

        Some(CollideResult{t, normal})
    }
}
