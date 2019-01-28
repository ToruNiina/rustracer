//! image stuff

use crate::error::Result;
use crate::color::{Color, RGBA, RGB};
use crate::util::clamp;
use std::io::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RGBPixel {
   r: u8,
   g: u8,
   b: u8,
}

impl RGBPixel {
    pub fn new(r: u8, g: u8, b: u8) -> RGBPixel {
        RGBPixel{r, g, b}
    }
}

impl std::convert::From<RGB> for RGBPixel {
    fn from(rgb: RGB) -> RGBPixel {
        RGBPixel::new(clamp(rgb.r() * 256.0, 0.0, 255.0) as u8,
                      clamp(rgb.g() * 256.0, 0.0, 255.0) as u8,
                      clamp(rgb.b() * 256.0, 0.0, 255.0) as u8)
    }
}

impl std::convert::From<RGBA> for RGBPixel {
    fn from(rgba: RGBA) -> RGBPixel {
        From::<RGB>::from(From::from(rgba))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Image {
    pixels: std::vec::Vec<RGBPixel>,
    width : usize,
    height: usize,
}

impl Image {
    pub fn new(w: usize, h: usize) -> Image {
        let mut v = std::vec::Vec::with_capacity(w * h);
        v.resize(w * h, RGBPixel::new(0, 0, 0));
        Image {
            pixels: v,
            width : w,
            height: h,
        }
    }

    pub fn at    (&self, w: usize, h: usize) -> &RGBPixel {
        &self.pixels[h * self.width + w]
    }
    pub fn at_mut(&mut self, w: usize, h: usize) -> &mut RGBPixel {
        &mut self.pixels[h * self.width + w]
    }

    pub fn lines(&self) -> std::slice::ChunksExact<RGBPixel> {
        self.pixels.chunks_exact(self.width)
    }
    pub fn lines_mut(&mut self) -> std::slice::ChunksExactMut<RGBPixel> {
        self.pixels.chunks_exact_mut(self.width)
    }

    pub fn rlines(&self) -> std::slice::RChunksExact<RGBPixel> {
        self.pixels.rchunks_exact(self.width)
    }
    pub fn rlines_mut(&mut self) -> std::slice::RChunksExactMut<RGBPixel> {
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
