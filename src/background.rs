use crate::vector::Vector3;
use crate::color::RGBA;

pub trait Background {
    fn color_at(&self, dir: Vector3) -> RGBA;
}

pub struct SkyBg;
impl Background for SkyBg {
    fn color_at(&self, dir: Vector3) -> RGBA {
        let t = dir.unit()[1] * 0.5 + 0.5;
        let u = 1.0 - t;
        RGBA::new(u + 0.5 * t, u + 0.7 * t, 1.0, 1.0)
    }
}

