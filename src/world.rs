use crate::color::RGB;
use crate::ray::Ray;
use crate::collide::{Collision, Collide};
use crate::material::{Scatter, Material};
use crate::object::Object;
use crate::background::Background;
use rand::Rng;

pub struct World<Bg> {
    pub objects: std::vec::Vec<Object>,
        bg:      Bg,
}

impl<Bg: Background> World<Bg> {
    pub fn new(objects: std::vec::Vec<Object>, bg: Bg) -> World<Bg> {
        World{objects, bg}
    }

    /// returns color & depth of the recursion.
    pub fn color<R>(&self, ray: Ray, rng: &mut R, depth: usize) -> (RGB, usize)
    where
        R: Rng
    {
        const DEPTH_LIMIT: usize = 100;
        if depth >= DEPTH_LIMIT {
            return (RGB::new(0.0, 0.0, 0.0), DEPTH_LIMIT);
        }

        let mut nearest = None;
        let mut min_t   = std::f32::INFINITY;
        for obj in self.objects.iter() {
            if let Some(collide) =
                obj.collide_within(&ray, 0.0001, std::f32::INFINITY) {

                if collide.t < min_t {
                    min_t = collide.t;
                    nearest = Some((obj, collide))
                }
            }
        }

        if let Some((nearest, collide)) = nearest {
            let next_ray = nearest.scatter(&ray, collide, rng);
            let (c, d)   = self.color(next_ray, rng, depth+1);
            (nearest.emission + nearest.albedo * c, d)
        } else {
            (self.bg.color_at(ray.direction), depth)
        }
    }
}
