use crate::vector::Vector3;
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

pub struct Screen<B: Background> {
    lower_left: Vector3,
    horizontal: Vector3,
    vertical:   Vector3,
    width:      usize,
    height:     usize,
    background: B,
}

impl<B: Background> Screen<B> {
    pub fn new(lower_left: Vector3, horizontal: Vector3, vertical:   Vector3,
               width:      usize,   height:     usize,   background: B)
        -> Screen<B> {
        Screen{lower_left, horizontal, vertical, width, height, background}
    }

    pub fn image(&self) -> Image {
        self.background.clear(self.width, self.height)
    }
}
