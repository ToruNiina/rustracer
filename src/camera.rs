use crate::vector::{Vector3, pick_in_circle};
use crate::world::World;
use crate::image::Image;
use crate::color::RGB;
use crate::ray::Ray;
use crate::background::Background;
use rand_core::SeedableRng;
use rand::Rng;

pub struct Camera {
    location:    Vector3,
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
}

impl Camera {
    pub fn new(location:   Vector3,
               direction:  Vector3,
               view_up:    Vector3,
               vertical_fov:   f32,
               aperture:       f32,
               focus_dist:     f32,
               width:        usize,
               height:       usize) -> Camera {

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
        let lower_left = location - focus_dist * screen_offset;

        let horizontal = focus_dist * 2.0 * half_width  * u;
        let vertical   = focus_dist * 2.0 * half_height * v;

        Camera{location,
               lower_left,
               horizontal,
               vertical,
               u, v,
               width,
               height,
               rwidth:  1.0 / width as f32,
               rheight: 1.0 / height as f32,
               lens_radius}
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

    fn ray_through_lens<R: Rng>(&self, w: usize, h: usize, n: usize, rng: &mut R)
        -> std::vec::Vec<Ray>
    {
        (0..n).map(|_| {
            let (cx, cy) = pick_in_circle(rng);
            let lens_offset = self.lens_radius * cx * self.u +
                              self.lens_radius * cy * self.v;

            let src = self.location + lens_offset;
            let dst = self.point_at_ratio(
                (w as f32 + rng.gen_range(0.0f32, 1.0f32)) * self.rwidth,
                (h as f32 + rng.gen_range(0.0f32, 1.0f32)) * self.rheight);

            Ray::new(src, dst - src)

        }).collect()
    }

    pub fn render<Bg: Background>(&self, world: World<Bg>) -> Image {
        const N:usize = 100;
        let mut img = Image::new(self.width, self.height);
        let mut rng = rand_xorshift::XorShiftRng::from_rng(
            rand_os::OsRng::new().unwrap()).unwrap();

        for w in 0..self.width {
            for h in 0..self.height {
                let color = self.ray_through_lens(w, h, N, &mut rng).into_iter()
                    .map(|ray| world.color(ray, &mut rng, 0).0)
                    .fold(RGB::new(0.0, 0.0, 0.0), |l, r| l + r) / (N as f32);

                *img.at_mut(w, h) = std::convert::From::from(color.sqrt());
            }
        }
        img
    }
}

pub struct CameraBuilder {
    loc: std::option::Option<Vector3>,
    dir: std::option::Option<Vector3>,
    vup: std::option::Option<Vector3>,
    aov: std::option::Option<f32>,
    apa: std::option::Option<f32>,
    fd:  std::option::Option<f32>,
    w:   std::option::Option<usize>,
    h:   std::option::Option<usize>,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            loc: None,
            dir: None,
            vup: None,
            aov: None,
            apa: None,
            fd:  None,
            w:   None,
            h:   None,
        }
    }

    pub fn position(mut self, loc: Vector3) -> Self {
        self.loc = Some(loc);
        self
    }
    pub fn direction(mut self, dir: Vector3) -> Self {
        self.dir = Some(dir);
        self
    }
    pub fn view_up(mut self, vup: Vector3) -> Self {
        self.vup = Some(vup);
        self
    }
    pub fn vertical_angle_of_view(mut self, aov: f32) -> Self {
        self.aov = Some(aov);
        self
    }
    pub fn diameter_of_apature(mut self, apa: f32) -> Self {
        self.apa = Some(apa);
        self
    }
    pub fn focus_distance(mut self, fd: f32) -> Self {
        self.fd = Some(fd);
        self
    }
    pub fn width(mut self, w: usize) -> Self {
        self.w = Some(w);
        self
    }
    pub fn height(mut self, h: usize) -> Self {
        self.h = Some(h);
        self
    }
    pub fn build(self) -> Camera {
        Camera::new(self.loc.unwrap(),
                    self.dir.unwrap(),
                    self.vup.unwrap(),
                    self.aov.unwrap(),
                    self.apa.unwrap(),
                    self.fd.unwrap(),
                    self.w.unwrap(),
                    self.h.unwrap())
    }
}



