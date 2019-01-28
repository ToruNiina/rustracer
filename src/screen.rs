use crate::vector::{Vector3, Vector4, pick_in_circle};
use crate::world::World;
use crate::image::{Color, Image};
use crate::ray::Ray;
use crate::background::Background;
use rand::Rng;

pub struct Screen<B: Background> {
    camera:      Vector3,
    lower_left:  Vector3,
    horizontal:  Vector3,
    vertical:    Vector3,
    u:           Vector3, // is a normalized horizontal
    v:           Vector3, // is a normalized vertical
    width:       usize,
    height:      usize,
    rwidth:      f32,
    rheight:     f32,
    lens_radius: f32,
    background:  B,
}

impl<B: Background> Screen<B> {
    pub fn new(camera:     Vector3,
               direction:  Vector3,
               view_up:    Vector3,
               vertical_fov:   f32,
               aperture:       f32,
               focus_dist:     f32,
               width:        usize,
               height:       usize,
               background: B) -> Screen<B> {

        let lens_radius  = aperture * 0.5;

        let aspect_ratio = width as f32 / height as f32;
        let theta        = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height  = (theta * 0.5).tan();
        let half_width   = half_height * aspect_ratio;

        let w = -Vector3::unit(direction);
        let u =  Vector3::unit(Vector3::cross(view_up, w));
        let v =  Vector3::cross(w, u);

        // the virtual screen plane is now at the `focus_dist` distant from camera

        let screen_offset = half_width * u + half_height * v + w;
        let lower_left = camera - focus_dist * screen_offset;

        let horizontal = focus_dist * 2.0 * half_width  * u;
        let vertical   = focus_dist * 2.0 * half_height * v;

        Screen{camera,
               lower_left,
               horizontal,
               vertical,
               u, v,
               width,
               height,
               rwidth:  1.0 / width as f32,
               rheight: 1.0 / height as f32,
               lens_radius,
               background}
    }


    fn point_at_ratio(&self, w: f32, h: f32) -> Vector3 {
        self.lower_left + w * self.horizontal + h * self.vertical
    }
//     fn point_at_pixel(&self, w: usize, h: usize) -> Vector3 {
//         let wr = (w as f32 + 0.5) * self.rwidth;
//         let hr = (h as f32 + 0.5) * self.rheight;
//         self.point_at_ratio(wr, hr)
//     }
//     fn grid_at_pixel(&self, w: usize, h: usize) -> [Vector3; 4] {
//         [
//             self.point_at_ratio((w as f32 + 0.25) * self.rwidth,
//                                 (h as f32 + 0.25) * self.rheight),
//             self.point_at_ratio((w as f32 + 0.25) * self.rwidth,
//                                 (h as f32 + 0.75) * self.rheight),
//             self.point_at_ratio((w as f32 + 0.75) * self.rwidth,
//                                 (h as f32 + 0.25) * self.rheight),
//             self.point_at_ratio((w as f32 + 0.75) * self.rwidth,
//                                 (h as f32 + 0.75) * self.rheight)
//         ]
//     }
//     fn random_at_pixel(&self, w: usize, h: usize, n: usize,
//                        rng: &mut rand::rngs::ThreadRng) -> std::vec::Vec<Vector3>
//     {
//         (0..n).map(|_| {
//             let u = rng.gen_range(0.0f32, 1.0f32);
//             let v = rng.gen_range(0.0f32, 1.0f32);
//             self.point_at_ratio((w as f32 + u) * self.rwidth,
//                                 (h as f32 + v) * self.rheight)
//         }).collect()
//     }

    fn ray_through_lens(&self, w: usize, h: usize, n: usize,
                        rng: &mut rand::rngs::ThreadRng) -> std::vec::Vec<Ray>
    {
        (0..n).map(|_| {
            let (cx, cy) = pick_in_circle(rng);
            let lens_offset = self.lens_radius * cx * self.u +
                              self.lens_radius * cy * self.v;

            let src = self.camera + lens_offset;
            let dst = self.point_at_ratio(
                (w as f32 + rng.gen_range(0.0f32, 1.0f32)) * self.rwidth,
                (h as f32 + rng.gen_range(0.0f32, 1.0f32)) * self.rheight);

            Ray::new(src, dst - src)

        }).collect()
    }

    pub fn render(&self, world: World) -> Image {
        const N:usize = 100;
        let mut img = Image::new(self.width, self.height);
        let mut rng = rand::thread_rng();

        for w in 0..self.width {
            for h in 0..self.height {
                let color = self.ray_through_lens(w, h, N, &mut rng).into_iter()
                    .map(|ray| world.color(&ray, &self.background, &mut rng, 0))
                    .fold(Vector4::zero(), |l, r| l + r) / (N as f32);

                *img.at_mut(w, h) =
                    Color::ratio(color[0].sqrt(), color[1].sqrt(), color[2].sqrt(), color[3]);
            }
        }
        img
    }
}
