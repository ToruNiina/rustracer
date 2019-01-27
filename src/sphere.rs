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
        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            None
        } else {
            let t = -b - d.sqrt() / (2.0 * a);
            let n = Vector3::unit(ray.at(t) - self.center);
            Some(CollideResult{t: t, normal: n})
        }
    }
}
