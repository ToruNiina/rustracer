use crate::vector::Vector3;
use crate::image::{Color, Image};
use crate::background::Background;

pub struct Screen<B: Background> {
    camera:     Vector3,
    lower_left: Vector3,
    horizontal: Vector3,
    vertical:   Vector3,
    width:      usize,
    height:     usize,
    background: B,
}

impl<B: Background> Screen<B> {
    pub fn new(camera:     Vector3,
               lower_left: Vector3,
               horizontal: Vector3,
               vertical:   Vector3,
               width:      usize,
               height:     usize,
               background: B)
        -> Screen<B> {
        Screen{camera, lower_left, horizontal, vertical, width, height, background}
    }

    pub fn image(&self) -> Image {
        self.background.clear(self.width, self.height)
    }
}
