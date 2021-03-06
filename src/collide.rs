use crate::vector::Vector3;
use crate::ray::Ray;

pub struct Collision {
    pub t: f32,
    pub normal: Vector3,
}

pub trait Collide {
    fn collide_within(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Collision>;
}
