use crate::vector::Vector3;
use crate::world::World;
use crate::image::{Color, Image};
use crate::ray::Ray;
use crate::collide::{CollideResult, Collide};
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

        for w in 0..self.width {
            for h in 0..self.height {
                let (r, g, b, a) = self.background
                    .color_ratio_at((w, self.width), (h, self.height));

                let mut count = 0usize;
                let mut r_sum = 0.0f32;
                let mut g_sum = 0.0f32;
                let mut b_sum = 0.0f32;
                let mut a_sum = 0.0f32;

                for p in self.grid_at_pixel(w, h).into_iter() {
                    let mut color = (r, g, b, a);
                    count += 1;

                    let ray = Ray::new(self.camera, *p - self.camera);

                    let mut min_t = std::f32::INFINITY;
                    for obj in world.objects.iter() {
                        if let Some(CollideResult{t, normal}) = obj.collide(&ray) {
                            if t < min_t {
                                min_t = t;
                                color = (normal[0] * 0.5 + 0.5,
                                         normal[1] * 0.5 + 0.5,
                                         normal[2] * 0.5 + 0.5, 1.0);
                            }
                        }
                    }
                    r_sum += color.0;
                    g_sum += color.1;
                    b_sum += color.2;
                    a_sum += color.3;
                }
                *img.at_mut(w, h) = Color::ratio(
                    r_sum / count as f32,
                    g_sum / count as f32,
                    b_sum / count as f32,
                    1.0);
            }
        }
        img
    }
}
