mod error;
mod image;
mod vector;
mod ray;
mod screen;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min } else if x > max { max } else { x }
}

struct SkyBg;
impl screen::Background for SkyBg {
    fn color_at(&self, _w: (usize, usize), h: (usize, usize)) -> image::Pixel {
        let t = h.0 as f64 / h.1 as f64;
        let r = clamp((1.0 - t) * 256.0 + t * 128.0, 0.0, 255.0) as u8;
        let g = clamp((1.0 - t) * 256.0 + t * 180.0, 0.0, 255.0) as u8;
        image::Pixel::rgb(r, g, 255)
    }
}

fn main() {
    let screen = screen::Screen::new(
        vector::Vector3::new(-2.0, -1.0, -1.0),
        vector::Vector3::new( 4.0,  0.0,  0.0),
        vector::Vector3::new( 0.0,  2.0,  0.0),
        640, 480, SkyBg);

    screen.image().write_ppm("example.ppm").unwrap();
}
