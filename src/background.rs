use crate::image::{Pixel, Image};

pub trait Background {
    fn color_at(&self, w: (usize, usize), h: (usize, usize)) -> Pixel;

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
