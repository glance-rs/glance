//! A high-level image handling module
//!
//! This crate provides an [`Image`] struct that wraps [`image::DynamicImage`] (from the [`image`]
//! crate) with additional functionality for display and basic image operations.

use crate::Result;

use image::{DynamicImage, EncodableLayout, GenericImageView};
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
        let width = self.dimensions().0 as usize;
        let height = self.dimensions().1 as usize;

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

    pub fn into_grayscale(&self) -> Result<Self> {
        Ok(Image {
            image: self.image.grayscale(),
        })
    }

    /// Returns true if the image contains no pixel data.
    pub fn is_empty(&self) -> bool {
        self.image.to_rgb8().is_empty()
    }

    /// Returns the image dimensions as (width, height).
    pub fn dimensions(&self) -> (u32, u32) {
        self.image.dimensions()
    }
}
