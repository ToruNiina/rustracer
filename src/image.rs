//! image stuff

use crate::error::Result;
use crate::util::clamp;
use std::io::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color {
   r: u8,
   g: u8,
   b: u8,
   a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {r:r, g:g, b:b, a:a}
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color {r:r, g:g, b:b, a:255}
    }

    pub fn ratio(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r: clamp(r * 256.0, 0.0, 255.0) as u8,
            g: clamp(g * 256.0, 0.0, 255.0) as u8,
            b: clamp(b * 256.0, 0.0, 255.0) as u8,
            a: clamp(a * 256.0, 0.0, 255.0) as u8,
        }
    }
}

impl std::default::Default for Color {
    fn default() -> Color {
        Color {r:0, g:0, b:0, a:0}
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Image {
    pixels: std::vec::Vec<Color>,
    width : usize,
    height: usize,
}

impl Image {
    pub fn new(w: usize, h: usize) -> Image {
        let mut v = std::vec::Vec::with_capacity(w * h);
        v.resize(w * h, Color::new(0, 0, 0, 0));
        Image {
            pixels: v,
            width : w,
            height: h,
        }
    }

    pub fn at    (&self, w: usize, h: usize) -> &Color {
        &self.pixels[h * self.width + w]
    }
    pub fn at_mut(&mut self, w: usize, h: usize) -> &mut Color {
        &mut self.pixels[h * self.width + w]
    }

    pub fn lines(&self) -> std::slice::ChunksExact<Color> {
        self.pixels.chunks_exact(self.width)
    }
    pub fn lines_mut(&mut self) -> std::slice::ChunksExactMut<Color> {
        self.pixels.chunks_exact_mut(self.width)
    }

    pub fn rlines(&self) -> std::slice::RChunksExact<Color> {
        self.pixels.rchunks_exact(self.width)
    }
    pub fn rlines_mut(&mut self) -> std::slice::RChunksExactMut<Color> {
        self.pixels.rchunks_exact_mut(self.width)
    }

    pub fn write_ppm<P>(&self, path: P) -> Result<()>
    where
        P: std::convert::AsRef<std::path::Path>
    {
        let file = std::fs::OpenOptions::new()
            .write(true).create(true).open(&path)?;

        let mut writer = std::io::BufWriter::new(file);

        writer.write(b"P6\n")?;
        writer.write_fmt(format_args!("{} {}\n255\n", self.width, self.height))?;

        // In the most image viewer, the origin locates upper left.
        // To output physical screen into an image, we need to flip y-axis.
        // So here it uses rlines.
        for line in self.rlines() {
            for pixel in line {
                writer.write(&[pixel.r, pixel.g, pixel.b])?;
            }
        }
        Ok(())
    }
}
