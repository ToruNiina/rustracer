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
    let screen = screen::Screen::new(
        /* camera position  */ vector::Vector3::new(-2.0,  2.0,  1.0),
        /* camera direction */ vector::Vector3::new( 2.0, -2.0, -2.0),
        /* camera view-up   */ vector::Vector3::new( 0.0,  1.0,  0.0),
        /* vertical angle of view */ 30.0,
        /* diameter of aperture   */ 0.5,
        /* focus distance         */ vector::Vector3::new( 2.0, -2.0, -2.0).len(),
        640, 320);

    let world = world::World::new(vec![
        world::Object::make_sphere(
            vector::Vector3::new(0.0,    0.0, -1.0),   0.5,
            material::Material::make_diffuse(0.8, 0.3, 0.3)),

        world::Object::make_sphere(
            vector::Vector3::new(0.0, -100.5, -1.0), 100.0,
            material::Material::make_diffuse(0.5, 0.5, 0.5)),

        world::Object::make_sphere(
            vector::Vector3::new(1.0,    0.0, -1.0),   0.5,
            material::Material::make_metalic(0.3, 0.8, 0.6, 0.2)),

        world::Object::make_sphere(
            vector::Vector3::new(-1.0,   0.0, -1.0),   0.5,
            material::Material::make_dielectric(1.5, 1.0, 1.0, 1.0)),
        world::Object::make_sphere(
            vector::Vector3::new(-1.0,   0.0, -1.0),   -0.45,
            material::Material::make_dielectric(1.5, 1.0, 1.0, 1.0)),
    ], background::SkyBg);

    screen.render(world).write_ppm("example.ppm").unwrap();
}
