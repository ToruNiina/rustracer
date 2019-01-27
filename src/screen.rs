use crate::vector::Vector3;
use crate::world::World;
use crate::image::{Color, Image};
use crate::ray::Ray;
use crate::collide::{CollideResult, Collide};
use crate::background::Background;
use crate::util::clamp;

pub struct Screen<B: Background> {
    camera:     Vector3,
    lower_left: Vector3,
    horizontal: Vector3,
    vertical:   Vector3,
    width:      usize,
    height:     usize,
    rwidth:     f32,
    rheight:    f32,
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
        Screen{camera,
               lower_left,
               horizontal,
               vertical,
               width,
               height,
               rwidth:  1.0 / width as f32,
               rheight: 1.0 / height as f32,
               background}
    }

    fn pixel_at(&self, w: usize, h: usize) -> Vector3 {
        let wr = w as f32 * self.rwidth;
        let hr = h as f32 * self.rheight;
        self.lower_left + wr * self.horizontal + hr * self.vertical
    }

    pub fn render(&self, world: World) -> Image {
        let mut img = self.background.clear(self.width, self.height);

        for w in 0..self.width {
            for h in 0..self.height {
                let ray = Ray::new(self.camera,
                                   self.pixel_at(w, h) - self.camera);
                for obj in world.objects.iter() {
                    if let Some(CollideResult{t:_, normal: n}) = obj.collide(&ray) {
                        let clr = Color::rgb(
                            clamp(0.5 * 256.0 * (n[0] + 1.0), 0.0, 255.0) as u8,
                            clamp(0.5 * 256.0 * (n[1] + 1.0), 0.0, 255.0) as u8,
                            clamp(0.5 * 256.0 * (n[2] + 1.0), 0.0, 255.0) as u8,
                        );
                        *img.at_mut(w, h) = clr;
                    }
                }
            }
        }
        img
    }
}
