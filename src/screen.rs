use crate::vector::Vector3;
use crate::world::World;
use crate::image::Image;
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

    pub fn render(&self, w: World) -> Image {
        self.background.clear(self.width, self.height)
    }
}
