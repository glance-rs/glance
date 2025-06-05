pub mod pixel;

use std::path::Path;

use crate::{CoreError, Result, drawing::traits::Drawable};
use image::{ImageBuffer, ImageReader, Rgba as ImageRgba};
use minifb::{Key, Window, WindowOptions};
use pixel::Pixel;

pub struct Image<P: Pixel> {
    width: usize,
    height: usize,
    data: Vec<P>,
}

impl<P> Image<P>
where
    P: Pixel,
{
    pub fn open<Pth: AsRef<Path>>(path: Pth) -> Result<Self> {
        let image = ImageReader::open(path)?.decode()?.to_rgba8();
        let (width, height) = image.dimensions();
        let width = width as usize;
        let height = height as usize;

        let data: Result<Vec<P>> = image.pixels().map(|p| P::from_rgba8(p.0)).collect();
        let data = data?;

        Ok(Image {
            width,
            height,
            data,
        })
    }

    pub fn save<Pth: AsRef<Path>>(&self, path: Pth) -> Result<()> {
        let rgba8_data: Vec<[u8; 4]> = self.data.iter().map(|pixel| pixel.to_rgba8()).collect();
        let rgba8_bytes: Vec<u8> = rgba8_data.iter().flatten().copied().collect();

        let buffer = ImageBuffer::<ImageRgba<u8>, _>::from_raw(
            self.width as u32,
            self.height as u32,
            rgba8_bytes,
        )
        .ok_or_else(|| std::io::Error::other("Invalid buffer"))?;
        buffer.save(path)?;

        Ok(())
    }

    pub fn display(&self, title: &str) -> Result<()> {
        let (width, height) = self.dimensions();

        // Create window
        let mut window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                resize: false,
                ..Default::default()
            },
        )?;
        window.set_target_fps(1);

        // Populate framebuffer
        let rgba8_data: Vec<[u8; 4]> = self.data.iter().map(|px| px.to_rgba8()).collect();

        let mut buffer: Vec<u32> = Vec::with_capacity(rgba8_data.len());
        for pixel in rgba8_data.iter() {
            buffer.push(u32::from_be_bytes([pixel[3], pixel[0], pixel[1], pixel[2]]));
        }

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buffer, width, height)?;
        }

        Ok(())
    }

    pub fn get_pixel(&self, position: (usize, usize)) -> Result<&P> {
        let idx = position.1 * self.width + position.0;
        self.data.get(idx).ok_or_else(|| {
            CoreError::OutOfBounds(format!(
                "{:#?} is out of bounds for image of size {:#?}",
                position,
                self.dimensions()
            ))
        })
    }

    pub fn set_pixel(&mut self, position: (usize, usize), color: P) -> Result<()> {
        let idx = position.1 * self.width + position.0;
        if let Some(px) = self.data.get_mut(idx) {
            *px = color;
        }
        Ok(())
    }

    pub fn draw<D: Drawable<P>>(&mut self, shape: D) -> Result<()> {
        shape.draw_on(self)?;
        Ok(())
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
