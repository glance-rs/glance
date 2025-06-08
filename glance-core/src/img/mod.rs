//! This module provides the [`Image`] struct for handling images with generic pixel data.
//! It includes functionality for loading, saving, displaying images, and manipulating pixel data.
//! The pixel data is represented by a type that implements the [`Pixel`] trait, allowing for
//! flexible support of different pixel formats like Rgba and Luma, with u8, u16, f32 (for now).
//!
//! ## Examples
//!
//! ```
//! use glance_core::img::{Image, pixel::Rgba};
//!
//! // Load an image. Type annotations are required for the pixel type. (Might change in the
//! // future)
//! if let Ok(image)= Image::<Rgba<u8>>::open("input.png") {
//!     let _ = image.display("My Image");
//! }
//! ```
pub mod iterators;
pub mod pixel;

use std::path::Path;

use crate::{CoreError, Result, drawing::traits::Drawable};
use image::{ImageBuffer, ImageReader, Rgba as ImageRgba};
use minifb::{Key, Window, WindowOptions};
use pixel::{Pixel, Rgba};

/// Image struct represents an image with pixel data of type P
/// where P implements the [`Pixel`] trait.
pub struct Image<P: Pixel> {
    width: usize,
    height: usize,
    data: Vec<P>,
}

impl<P> Image<P>
where
    P: Pixel,
{
    /// Creates a new empty [`Image`] instance with the specified width and height.
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            data: vec![P::from_rgba8([0, 0, 0, 0]).unwrap(); width * height],
        }
    }

    /// Creates a new [`Image`] instance from the given width, height, and pixel data.
    pub fn from_data(width: usize, height: usize, data: Vec<P>) -> Result<Self> {
        if data.len() != width * height {
            return Err(CoreError::InvalidData(
                "Data length does not match width * height".to_string(),
            ));
        }
        Ok(Image {
            width,
            height,
            data,
        })
    }

    /// Creates a new [`Image`] instance from the given path.
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

    /// Saves the image to the specified path. File format is determined by the file extension.
    /// See [`image::ImageBuffer::save`] for more details.
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

    /// Opens an [`Image`] instance and displays it in a window.
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
        window.set_target_fps(30);

        // Populate framebuffer
        let buffer: Vec<u32> = self
            .data
            .iter()
            .map(|px| {
                let rgba = px.to_rgba8();
                if rgba[3] == 0 {
                    return 0; // Transparent pixel
                }
                u32::from_be_bytes([rgba[3], rgba[0], rgba[1], rgba[2]])
            })
            .collect();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buffer, width, height)?;
        }

        Ok(())
    }

    /// Returns a reference to the pixel data at the specified position.
    /// Returns an error if the position is out of bounds.
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

    /// Sets the pixel at the specified position to the given color.
    /// Colors are of type P, which implements the [`Pixel`] trait.
    /// Returns an error if the position is out of bounds.
    pub fn set_pixel(&mut self, position: (usize, usize), color: P) -> Result<()> {
        let idx = position.1 * self.width + position.0;
        if let Some(px) = self.data.get_mut(idx) {
            *px = color;
        } else {
            return Err(CoreError::OutOfBounds(format!(
                "{:#?} is out of bounds for image of size {:#?}",
                position,
                self.dimensions()
            )));
        }
        Ok(())
    }

    /// Draws a shape on the image. The shape must implement the [`Drawable`] trait.
    pub fn draw<D: Drawable<P>>(&mut self, shape: D) -> Result<()> {
        shape.draw_on(self)?;
        Ok(())
    }

    /// Returns the dimensions of the image as a tuple (width, height).
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Returns true if the image is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Convert the image to RGBA8 format.
    pub fn to_rgba8(&self) -> Image<Rgba<u8>> {
        let rgba_data: Vec<Rgba<u8>> = self
            .data
            .iter()
            .map(|px| {
                let rgba = px.to_rgba8();
                Rgba {
                    r: rgba[0],
                    g: rgba[1],
                    b: rgba[2],
                    a: rgba[3],
                }
            })
            .collect();

        Image {
            width: self.width,
            height: self.height,
            data: rgba_data,
        }
    }
}
