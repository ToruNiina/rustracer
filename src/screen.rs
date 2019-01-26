use crate::vector::Vector3;
use crate::image::{Pixel, Image};
use crate::background::Background;

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
