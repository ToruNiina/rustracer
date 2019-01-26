mod error;
mod image;
mod vector;
mod ray;
mod screen;
mod background;


fn main() {
    let screen = screen::Screen::new(
        vector::Vector3::new(-2.0, -1.0, -1.0),
        vector::Vector3::new( 4.0,  0.0,  0.0),
        vector::Vector3::new( 0.0,  2.0,  0.0),
        640, 480, background::SkyBg);

    screen.image().write_ppm("example.ppm").unwrap();
}
