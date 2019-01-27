use crate::vector::Vector3;
use crate::ray::Ray;

pub struct CollideResult {
    pub t: f32,
    pub normal: Vector3,
}

pub trait Collide {
    fn collide(&self, ray: &Ray) -> Option<CollideResult>;

    fn collide_within(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<CollideResult> {
        self.collide(ray).map_or(None, |cr| if cr.t < t_min || t_max < cr.t {
                None
            } else {
                Some(cr)
            })
    }
}
