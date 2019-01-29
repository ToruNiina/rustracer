mod util;
mod error;
mod image;
mod vector;
mod ray;
mod camera;
mod sphere;
mod collide;
mod material;
mod background;
mod world;
mod color;

fn main() {
    use crate::vector::Vector3;
    use crate::material::Material;
    use crate::sphere::Sphere;
    use crate::color::RGB;

    let camera = camera::Camera::new(
        /* camera position        */ Vector3::new(-2.0,  0.0,  1.0),
        /* camera direction       */ Vector3::new( 2.0,  0.2, -2.0),
        /* camera view-up         */ Vector3::new( 0.0,  1.0,  0.0),
        /* vertical angle of view */ 90.0,
        /* diameter of aperture   */ 0.01,
        /* focus distance         */ Vector3::new( 2.0, -2.0, -2.0).len(),
        640, 320);

    let world = world::World::new(vec![
        world::Object::make_sphere(
            Sphere::new(Vector3::new(0.0, 1.01, -1.0), 0.5),
            Material::make_diffuse(),
            RGB::new(1.0, 1.0, 1.0),
            RGB::new(0.4, 0.4, 0.0)),

        world::Object::make_sphere(
            Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5),
            Material::make_diffuse(),
            RGB::new(0.8, 0.3, 0.3),
            RGB::new(0.0, 0.0, 0.0)),

        world::Object::make_sphere(
            Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0),
            Material::make_diffuse(),
            RGB::new(0.5, 0.5, 0.5),
            RGB::new(0.0, 0.0, 0.0)),

        world::Object::make_sphere(
            Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5),
            Material::make_metalic(0.3),
            RGB::new(0.8, 0.6, 0.2),
            RGB::new(0.0, 0.0, 0.0)),

        world::Object::make_sphere(
            Sphere::new(Vector3::new(-1.0, 0.0, -1.0),  0.5),
            Material::make_dielectric(1.5),
            RGB::new(1.0, 1.0, 1.0),
            RGB::new(0.0, 0.0, 0.0)),

        world::Object::make_sphere(
            Sphere::new(Vector3::new(-1.0, 0.0, -1.0), -0.45),
            Material::make_dielectric(1.5),
            RGB::new(1.0, 1.0, 1.0),
            RGB::new(0.0, 0.0, 0.0)),
    ], background::UniBg::new(RGB::new(0.5, 0.5, 0.5)));

    camera.render(world).write_ppm("example.ppm").unwrap();
}
