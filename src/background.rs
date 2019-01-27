use crate::image::{Color, Image};
use crate::util::clamp;

pub trait Background {
    fn color_at(&self, w: (usize, usize), h: (usize, usize)) -> Color;

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
    fn color_at(&self, _w: (usize, usize), h: (usize, usize)) -> Color {
        let t = h.0 as f64 / h.1 as f64;
        let r = clamp((1.0 - t) * 256.0 + t * 128.0, 0.0, 255.0) as u8;
        let g = clamp((1.0 - t) * 256.0 + t * 180.0, 0.0, 255.0) as u8;
        Color::rgb(r, g, 255)
    }
}

