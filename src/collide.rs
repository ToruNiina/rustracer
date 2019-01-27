use crate::vector::Vector3;
use crate::ray::Ray;

pub struct CollideResult {
    pub t: f32,
    pub normal: Vector3,
}

pub trait Collide {
    fn collide(&self, ray: &Ray) -> Option<CollideResult>;
}
