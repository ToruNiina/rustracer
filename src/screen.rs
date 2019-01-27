use crate::vector::{Vector3, Vector4};
use crate::world::World;
use crate::image::{Color, Image};
use crate::ray::Ray;
use crate::background::Background;

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

    fn point_at_ratio(&self, w: f32, h: f32) -> Vector3 {
        self.lower_left + w * self.horizontal + h * self.vertical
    }
    fn point_at_pixel(&self, w: usize, h: usize) -> Vector3 {
        let wr = (w as f32 + 0.5) * self.rwidth;
        let hr = (h as f32 + 0.5) * self.rheight;
        self.point_at_ratio(wr, hr)
    }
    fn grid_at_pixel(&self, w: usize, h: usize) -> [Vector3; 4] {
        [
            self.point_at_ratio((w as f32 + 0.25) * self.rwidth,
                                (h as f32 + 0.25) * self.rheight),
            self.point_at_ratio((w as f32 + 0.25) * self.rwidth,
                                (h as f32 + 0.75) * self.rheight),
            self.point_at_ratio((w as f32 + 0.75) * self.rwidth,
                                (h as f32 + 0.25) * self.rheight),
            self.point_at_ratio((w as f32 + 0.75) * self.rwidth,
                                (h as f32 + 0.75) * self.rheight)
        ]
    }

    pub fn render(&self, world: World) -> Image {
        let mut img = Image::new(self.width, self.height);
        let mut rng = rand::thread_rng();

        for w in 0..self.width {
            for h in 0..self.height {
                let color = self.grid_at_pixel(w, h).into_iter()
                    .map(|p| world.color(&Ray::new(self.camera, *p - self.camera),
                                         &self.background, &mut rng))
                    .fold(Vector4::zero(), |l, r| l + r) * 0.25;

                *img.at_mut(w, h) =
                    Color::ratio(color[0], color[1], color[2], color[3]);
            }
        }
        img
    }
}
