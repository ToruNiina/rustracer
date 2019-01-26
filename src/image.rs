//! image stuff

use crate::error::Result;
use std::io::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Pixel {
   r: u8,
   g: u8,
   b: u8,
   a: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel {r:r, g:g, b:b, a:a}
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {r:r, g:g, b:b, a:255}
    }
}

impl std::default::Default for Pixel {
    fn default() -> Pixel {
        Pixel {r:0, g:0, b:0, a:0}
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Image {
    pixels: std::vec::Vec<Pixel>,
    width : usize,
    height: usize,
}

impl Image {
    pub fn new(w: usize, h: usize) -> Image {
        let mut v = std::vec::Vec::with_capacity(w * h);
        v.resize(w * h, Pixel::new(0, 0, 0, 0));
        Image {
            pixels: v,
            width : w,
            height: h,
        }
    }

    pub fn lines(&self) -> std::slice::ChunksExact<Pixel> {
        self.pixels.chunks_exact(self.width)
    }
    pub fn lines_mut(&mut self) -> std::slice::ChunksExactMut<Pixel> {
        self.pixels.chunks_exact_mut(self.width)
    }

    pub fn rlines(&self) -> std::slice::RChunksExact<Pixel> {
        self.pixels.rchunks_exact(self.width)
    }
    pub fn rlines_mut(&mut self) -> std::slice::RChunksExactMut<Pixel> {
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
