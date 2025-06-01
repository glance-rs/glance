//! A high-level image handling module
//!
//! This crate provides an [`Image`] struct that can open, display and save images.

use crate::drawing::traits::Drawable;
use crate::utils;
use crate::{Error, Result};

use image::{ImageBuffer, ImageReader, Rgba};
use minifb::{Key, Window, WindowOptions};
use std::path::Path;

/// A struct that provides image handling functionality
///
/// # Examples
///
/// ```
/// // Open and display an image
/// use glance_core::img::Image;
///
/// if let Ok(img) = Image::open("path/to/image.jpg") {
///     let _ = img.display("Example Image");
/// }
/// ```
pub struct Image {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Image {
    /// Opens an image from the given path.
    ///
    /// Returns an error if the file does not exist or cannot be decoded.
    /// Supports all formats recognized by the `image` crate.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let dyn_img = ImageReader::open(path)?.decode()?;
        let rgba = dyn_img.to_rgba8();
        let (width, height) = rgba.dimensions();
        Ok(Image {
            width,
            height,
            data: rgba.into_raw(),
        })
    }

    /// Saves an image to the given path
    ///
    /// Returns an error if the file cannot be created or buffer is invalid.
    /// Format is recognized from file extension (see [`image::ImageBuffer::save`] for more info).
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let buffer: ImageBuffer<Rgba<u8>, _> =
            ImageBuffer::from_raw(self.width, self.height, self.data.clone())
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Invalid buffer"))?;

        buffer.save(path)?;

        Ok(())
    }

    /// Displays the image (as RGBA8) in a window until Escape is pressed.
    /// Returns an error if the window cannot be created.
    /// The window runs at 1 FPS to minimize CPU usage.
    /// Uses `minifb` for cross-platform windowing.
    pub fn display(&self, title: &str) -> Result<()> {
        let dims = self.dimensions();
        let width = dims[0] as usize;
        let height = dims[1] as usize;

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
        let rgba_bytes: &[u8] = &self.data;
        let mut buffer: Vec<u32> = Vec::with_capacity(rgba_bytes.len() / 4);
        for chunk in rgba_bytes.chunks(4) {
            buffer.push(u32::from_be_bytes([chunk[3], chunk[0], chunk[1], chunk[2]]));
        }

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buffer, width, height)?;
        }

        Ok(())
    }

    /// Gets the color of a pixel. Top left is treated as origin. Right is positive x, down is
    /// positive y.
    pub fn get_pixel(&self, position: [u32; 2]) -> Result<[u8; 4]> {
        let dims = self.dimensions();
        if position[0] >= dims[0] || position[1] >= dims[1] {
            return Err(Error::OutOfBounds(format!(
                "The image dimensions are {dims:?}. Getting pixel {position:?} is not possible."
            )));
        }

        let idx = ((position[1] * dims[0] + position[0]) * 4) as usize;
        let pixel = [
            self.data[idx],
            self.data[idx + 1],
            self.data[idx + 2],
            self.data[idx + 3],
        ];
        Ok(pixel)
    }

    /// Sets a pixel to the given color. Top left is treated as origin, x-axis goes horizontally.
    pub fn set_pixel(&mut self, position: [u32; 2], color: [u8; 4]) -> Result<()> {
        let dims = self.dimensions();
        if position[0] >= dims[0] || position[1] >= dims[1] {
            return Err(Error::OutOfBounds(format!(
                "The image dimensions are {dims:?}. Setting pixel {position:?} is not possible."
            )));
        }

        let idx = ((position[1] * dims[0] + position[0]) * 4) as usize;
        self.data[idx..idx + 4].copy_from_slice(&color);

        Ok(())
    }

    /// Linearly interpolate a pixel color with the given color
    pub fn alpha_blend_pixel(&mut self, position: [u32; 2], color: [u8; 4]) -> Result<()> {
        let dims = self.dimensions();
        if position[0] >= dims[0] || position[1] >= dims[1] {
            return Err(Error::OutOfBounds(format!(
                "The image dimensions are {dims:?}. Setting pixel {position:?} is not possible."
            )));
        }

        let color_fg = color;
        let color_bg = self.get_pixel(position)?;
        let blend_color = utils::alpha_blend(color_fg, color_bg);

        self.set_pixel(position, blend_color)?;

        Ok(())
    }

    /// Draw a shape (any struct that implements the [`drawing::traits::Drawable`] trait)
    pub fn draw<D: Drawable>(&mut self, shape: D) -> Result<()> {
        shape.draw_on(self)?;
        Ok(())
    }

    /// Returns true if the image contains no pixel data.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the image dimensions as (width, height).
    pub fn dimensions(&self) -> [u32; 2] {
        return [self.width, self.height];
    }
}
