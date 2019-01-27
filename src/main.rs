mod util;
mod error;
mod image;
mod vector;
mod ray;
mod screen;
mod sphere;
mod collide;
mod background;
mod world;

fn main() {
    let screen = screen::Screen::new(
        vector::Vector3::new( 0.0,  0.0,  0.0),
        vector::Vector3::new(-2.0, -1.0, -1.0),
        vector::Vector3::new( 4.0,  0.0,  0.0),
        vector::Vector3::new( 0.0,  2.0,  0.0),
        640, 320, background::SkyBg);

    let world = world::World::new(vec![
        world::Object::make_sphere(vector::Vector3::new(0.0, 0.0, -1.0), 0.5),
    ]);

    screen.render(world).write_ppm("example.ppm").unwrap();
}
