mod util;
mod error;
mod image;
mod vector;
mod ray;
mod screen;
mod sphere;
mod collide;
mod material;
mod background;
mod world;
mod color;

fn main() {
    use crate::vector::Vector3;
    use crate::material::Material;
    use crate::color::RGB;

    let screen = screen::Screen::new(
        /* camera position        */ Vector3::new(-2.0,  2.0,  1.0),
        /* camera direction       */ Vector3::new( 2.0, -2.0, -2.0),
        /* camera view-up         */ Vector3::new( 0.0,  1.0,  0.0),
        /* vertical angle of view */ 30.0,
        /* diameter of aperture   */ 0.5,
        /* focus distance         */ Vector3::new( 2.0, -2.0, -2.0).len(),
        640, 320);

    let world = world::World::new(vec![
        world::Object::make_sphere(
            Vector3::new(0.0, 0.0, -1.0), 0.5,
            Material::make_diffuse(0.8, 0.3, 0.3)),

        world::Object::make_sphere(
            Vector3::new(0.0, -100.5, -1.0), 100.0,
            Material::make_diffuse(0.5, 0.5, 0.5)),

        world::Object::make_sphere(
            Vector3::new(1.0, 0.0, -1.0), 0.5,
            Material::make_metalic(0.3, 0.8, 0.6, 0.2)),

        world::Object::make_sphere(
            Vector3::new(-1.0, 0.0, -1.0),  0.5,
            Material::make_dielectric(1.5, 1.0, 1.0, 1.0)),
        world::Object::make_sphere(
            Vector3::new(-1.0, 0.0, -1.0), -0.45,
            Material::make_dielectric(1.5, 1.0, 1.0, 1.0)),
    ], background::SkyBg);

    screen.render(world).write_ppm("example.ppm").unwrap();
}
