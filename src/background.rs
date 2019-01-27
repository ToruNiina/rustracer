use crate::image::{Color, Image};
use crate::util::clamp;

pub trait Background {
    fn color_ratio_at(&self, w: (usize, usize), h: (usize, usize))
        -> (f32, f32, f32, f32);

    fn color_at(&self, w: (usize, usize), h: (usize, usize)) -> Color {
        let (r, g, b, a) = self.color_ratio_at(w, h);
        Color::ratio(r, g, b, a)
    }

    fn clear(&self, w: usize, h: usize) -> Image {
        let mut image = Image::new(w, h);
        for i in 0..w {
            for j in 0..h {
                *image.at_mut(i, j) = self.color_at((i, w), (j, h));
            }
        }
        image
    }
}

pub struct SkyBg;

impl Background for SkyBg {
    fn color_ratio_at(&self, _w: (usize, usize), h: (usize, usize))
        -> (f32, f32, f32, f32)
    {
        let t = h.0 as f32 / h.1 as f32;
        let u = 1.0 - t;
        (u + 0.5 * t, u + 0.7 * t, 1.0, 1.0)
    }
}

