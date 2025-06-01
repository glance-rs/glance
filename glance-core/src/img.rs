//! A high-level image handling module
//!
//! This crate provides an [`Image`] struct that wraps [`image::DynamicImage`] (from the [`image`]
//! crate) with additional functionality for display and basic image operations.

use crate::drawing::traits::Drawable;
use crate::utils;
use crate::{Error, Result};

use image::{DynamicImage, EncodableLayout, GenericImage, GenericImageView, Rgba};
use minifb::{Key, Window, WindowOptions};
use std::path::Path;

/// A wrapper around dynamic image data with display capabilities
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
    image: DynamicImage,
}

impl Image {
    /// Opens an image from the given path.
    ///
    /// Returns an error if the file does not exist or cannot be decoded.
    /// Supports all formats recognized by the `image` crate.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let image = image::open(path)?;
        Ok(Image { image })
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
        let rgba_image = self.image.to_rgba8();
        let rgba_bytes: &[u8] = rgba_image.as_bytes();
        let mut buffer: Vec<u32> = Vec::with_capacity(rgba_bytes.len() / 4);
        for chunk in rgba_bytes.chunks(4) {
            buffer.push(u32::from_be_bytes([chunk[3], chunk[0], chunk[1], chunk[2]]));
        }

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buffer, width, height)?;
        }

        Ok(())
    }

    /// Gets the color of a pixel. Top left is treated as origin, x-axis goes horizontally.
    pub fn get_pixel(&self, position: [u32; 2]) -> Result<[u8; 4]> {
        let dims = self.dimensions();
        if position[0] >= dims[0] || position[1] >= dims[1] {
            return Err(Error::OutOfBounds(format!(
                "The image dimensions are {dims:?}. Getting pixel {position:?} is not possible."
            )));
        }

        Ok(self.image.get_pixel(position[0], position[1]).0)
    }

    /// Sets a pixel to the given color. Top left is treated as origin, x-axis goes horizontally.
    pub fn set_pixel(&mut self, position: [u32; 2], color: [u8; 4]) -> Result<()> {
        let dims = self.dimensions();
        if position[0] >= dims[0] || position[1] >= dims[1] {
            return Err(Error::OutOfBounds(format!(
                "The image dimensions are {dims:?}. Setting pixel {position:?} is not possible."
            )));
        }

        self.image.put_pixel(position[0], position[1], Rgba(color));
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

        self.image
            .put_pixel(position[0], position[1], Rgba(blend_color));

        Ok(())
    }

    /// Draw a shape (any struct that implements the [`drawing::traits::Drawable`] trait)
    pub fn draw<D: Drawable>(&mut self, shape: D) -> Result<()> {
        shape.draw_on(self)?;
        Ok(())
    }

    /// Returns a grayscaled image
    pub fn into_grayscale(&self) -> Result<Self> {
        Ok(Image {
            image: self.image.grayscale(),
        })
    }

    /// Returns true if the image contains no pixel data.
    pub fn is_empty(&self) -> bool {
        self.image.to_rgba8().is_empty()
    }

    /// Returns the image dimensions as (width, height).
    pub fn dimensions(&self) -> [u32; 2] {
        self.image.dimensions().into()
    }
}
