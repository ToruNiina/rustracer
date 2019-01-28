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
    fn collide_within(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<CollideResult> {
        let oc = ray.origin - self.center;
        let b  = 2.0 * Vector3::dot(oc, ray.direction);
        let c  = oc.len_sq() - self.radius * self.radius;
        let d  = b * b - 4.0 * c;

        if d < 0.0 {return None;}
        let sqrt_d = d.sqrt();

        let t = (-b - sqrt_d) * 0.5;
        if t_min <= t && t <= t_max {
            let normal = (ray.at(t) - self.center) / self.radius;
            return Some(CollideResult{t, normal})
        }

        let t = (-b + sqrt_d) * 0.5;
        if t_min <= t && t <= t_max {
            let normal = (ray.at(t) - self.center) / self.radius;
            return Some(CollideResult{t, normal})
        }
        None
    }
}
