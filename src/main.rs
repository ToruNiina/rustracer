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
mod object;

fn main() {
    use crate::vector::Vector3;
    use crate::material::Material;
    use crate::sphere::Sphere;
    use crate::color::RGB;

    let camera = camera::CameraBuilder::new()
        .position(Vector3::new(-2.0,  0.0,  1.0))
        .direction(Vector3::new( 2.0,  0.2, -2.0))
        .view_up(Vector3::new( 0.0,  1.0,  0.0))
        .vertical_angle_of_view(90.0)
        .diameter_of_apature(0.01)
        .focus_distance(Vector3::new( 2.0, -2.0, -2.0).len())
        .width(640)
        .height(320)
        .build();

    let world = world::World::new(vec![
        object::Object::make_sphere(
            Sphere::new(Vector3::new(0.0, 1.01, -1.0), 0.5),
            Material::make_diffuse(),
            RGB::new(1.0, 1.0, 1.0),
            RGB::new(0.4, 0.4, 0.0)),

        object::Object::make_sphere(
            Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5),
            Material::make_diffuse(),
            RGB::new(0.8, 0.3, 0.3),
            RGB::new(0.0, 0.0, 0.0)),

        object::Object::make_sphere(
            Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0),
            Material::make_diffuse(),
            RGB::new(0.5, 0.5, 0.5),
            RGB::new(0.0, 0.0, 0.0)),

        object::Object::make_sphere(
            Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5),
            Material::make_metalic(0.3),
            RGB::new(0.8, 0.6, 0.2),
            RGB::new(0.0, 0.0, 0.0)),

        object::Object::make_sphere(
            Sphere::new(Vector3::new(-1.0, 0.0, -1.0),  0.5),
            Material::make_dielectric(1.5),
            RGB::new(1.0, 1.0, 1.0),
            RGB::new(0.0, 0.0, 0.0)),

        object::Object::make_sphere(
            Sphere::new(Vector3::new(-1.0, 0.0, -1.0), -0.45),
            Material::make_dielectric(1.5),
            RGB::new(1.0, 1.0, 1.0),
            RGB::new(0.0, 0.0, 0.0)),
    ], background::UniBg::new(RGB::new(0.5, 0.5, 0.5)));

    camera.render(world).write_ppm("example.ppm").unwrap();
}
