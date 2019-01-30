use crate::vector::Vector3;
use crate::ray::Ray;
use crate::collide::{Collision, Collide};

pub struct Sphere {
    center: Vector3,
    radius: f32,
    rradius: f32,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Sphere {
        Sphere{center, radius, rradius: 1.0 / radius}
    }
}

impl Collide for Sphere {
    fn collide_within(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Collision> {
        let oc = ray.origin - self.center;
        let b  = Vector3::dot(oc, ray.direction);
        let c  = oc.len_sq() - self.radius * self.radius;
        if 0.0 < c && 0.0 < b {return None;}

        let d  = b * b - c;
        if d < 0.0 {return None;}

        let sqrt_d = d.sqrt();

        let t = (-b - sqrt_d);
        if t_min <= t && t <= t_max {
            let normal = (ray.at(t) - self.center) * self.rradius;
            return Some(Collision{t, normal})
        }

        let t = (-b + sqrt_d);
        if t_min <= t && t <= t_max {
            let normal = (ray.at(t) - self.center) * self.rradius;
            return Some(Collision{t, normal})
        }
        None
    }
}
