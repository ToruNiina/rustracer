use crate::image::{Color, Image};
use crate::vector::{Vector4, Vector3};
use crate::util::clamp;

pub trait Background {
    fn color_ratio_at(&self, dir: Vector3) -> Vector4;

    fn color_at(&self, dir: Vector3) -> Color {
        let color = self.color_ratio_at(dir);
        Color::ratio(color[0], color[1], color[2], color[3])
    }
}

pub struct SkyBg;

impl Background for SkyBg {
    fn color_ratio_at(&self, dir: Vector3) -> Vector4 {
        let t = dir.unit()[1] * 0.5 + 0.5;
        let u = 1.0 - t;
        Vector4::new(u + 0.5 * t, u + 0.7 * t, 1.0, 1.0)
    }
}

