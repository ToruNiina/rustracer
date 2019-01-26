mod error;
mod image;
mod vector;

fn main() {
    let mut img = image::Image::new(640, 480);

    for (i, line) in img.lines_mut().enumerate() {
        let ratio = i as f32 / 480.0;
        let inten = (256.0 * ratio) as u8;
        let color = image::Pixel::rgb(inten, inten, inten);
        for pixel in line.iter_mut() {
            *pixel = color;
        }
    }
    img.write_ppm("example.ppm").unwrap();
}
