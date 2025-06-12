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
//! if let Ok(image)= Image::<Rgba>::open("input.png") {
//!     let _ = image.display("My Image");
//! }
//! ```
pub mod iterators;
pub mod pixel;

use crate::{CoreError, Result, drawing::traits::Drawable};
use image::{ImageBuffer, ImageReader, Rgba as ImageRgba};
use minifb::{Key, Window, WindowOptions};
use pixel::{Luma, Pixel, Rgba};
use rayon::prelude::*;
use std::path::Path;

/// Image struct represents an image with pixel data of type P
/// where P implements the [`Pixel`] trait.
#[derive(Debug, Clone)]
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
            data: vec![P::new(); width * height],
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

        let data: Vec<P> = image.pixels().map(|p| P::from_rgba8(p.0)).collect();

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

    /// Fills the image with the specified color.
    pub fn fill(mut self, color: P) -> Self {
        self.data.fill(color);
        self
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

    /// Vertically stacks two images of the same width.
    pub fn vstack(mut self, other: &Self) -> Result<Self> {
        if self.width != other.width {
            return Err(CoreError::InvalidData(
                "Images must have the same width to stack vertically".to_string(),
            ));
        }

        self.height += other.height;
        self.data.extend(other.data.clone());
        Ok(self)
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
}

impl Image<Rgba> {
    /// Min-max normalizes the pixel data in the image.
    /// The alpha channel is not modified.
    pub fn normalize(&self) -> Self {
        // Find the maximum value in the pixel data for each channel
        let (max_r, max_g, max_b, _max_a) = self
            .par_pixels()
            .map(|pixel| (pixel.r, pixel.g, pixel.b, pixel.a))
            .reduce(
                || (0.0, 0.0, 0.0, 0.0),
                |(max_r, max_g, max_b, max_a), (r, g, b, a)| {
                    (max_r.max(r), max_g.max(g), max_b.max(b), max_a.max(a))
                },
            );

        let (min_r, min_g, min_b, _min_a) = self
            .par_pixels()
            .map(|pixel| (pixel.r, pixel.g, pixel.b, pixel.a))
            .reduce(
                || (f32::MAX, f32::MAX, f32::MAX, f32::MAX),
                |(min_r, min_g, min_b, min_a), (r, g, b, a)| {
                    (min_r.min(r), min_g.min(g), min_b.min(b), min_a.min(a))
                },
            );

        // Normalize each pixel
        let normalized = self
            .par_pixels()
            .map(|pixel| Rgba {
                r: (pixel.r - min_r) / (max_r - min_r),
                g: (pixel.g - min_g) / (max_g - min_g),
                b: (pixel.b - min_b) / (max_b - min_b),
                a: pixel.a,
            })
            .collect();

        Self {
            width: self.width,
            height: self.height,
            data: normalized,
        }
    }

    /// Applies a function to each pixel in the image, returning a new image.
    /// The alpha channel is not modified.
    pub fn apply<F>(mut self, f: F) -> Self
    where
        F: Fn(f32) -> f32 + Sync + Send,
    {
        self.par_pixels_mut()
            .for_each(|pixel| *pixel = pixel.apply(&f));
        self
    }
}

impl Image<Luma> {
    /// Min-max normalizes the pixel data in the image.
    pub fn normalize(&self) -> Self {
        // Find the maximum value in the pixel data for each channel
        let max_l = self
            .par_pixels()
            .map(|pixel| pixel.l)
            .reduce(|| 0.0, |max_l, l| (max_l.max(l)));

        let min_l = self
            .par_pixels()
            .map(|pixel| pixel.l)
            .reduce(|| f32::MAX, |min_l, l| (min_l.min(l)));
        // Normalize each pixel
        let normalized = self
            .par_pixels()
            .map(|pixel| Luma {
                l: (pixel.l - min_l) / (max_l - min_l),
            })
            .collect();

        Self {
            width: self.width,
            height: self.height,
            data: normalized,
        }
    }

    /// Applies a function to each pixel in the image, returning a new image.
    pub fn apply<F>(mut self, f: F) -> Self
    where
        F: Fn(f32) -> f32 + Sync + Send,
    {
        self.par_pixels_mut()
            .for_each(|pixel| *pixel = pixel.apply(&f));
        self
    }
}
