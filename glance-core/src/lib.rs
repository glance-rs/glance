pub mod drawing;
mod error;
pub mod img;

pub use self::error::{CoreError, Result};

#[cfg(test)]
mod tests {
    use rayon::iter::{IndexedParallelIterator, ParallelIterator};

    use super::*;
    use crate::drawing::shapes::Circle;
    use crate::img::Image;
    use crate::img::pixel::{Luma, Rgba};
    use std::path::PathBuf;

    // Open an image
    #[test]
    fn open_valid_image() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/lichtenstein.png");

        let img: Image<Rgba<u8>> = Image::open(&path)?;

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("open_valid_image")?;
        }

        assert!(!img.is_empty());
        Ok(())
    }

    // Open an invalid image path
    #[test]
    fn open_invalid_path() {
        let result = Image::<Rgba<u8>>::open("non_existent_file.jpg");
        assert!(result.is_err());
    }

    // Draw shapes on an image
    #[test]
    fn draw_shapes() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/eye_orange.png");

        let mut img = Image::open(&path)?;
        let dims = img.dimensions();

        let center = (dims.0 / 2, dims.1 / 2);
        let green = Rgba {
            r: 0u8,
            g: 255u8,
            b: 0u8,
            a: 150u8,
        };

        img.draw(Circle {
            position: center,
            color: green,
            radius: 100,
            filled: true,
            thickness: 5,
        })?;

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("draw_shapes")?;
        }

        assert!(img.get_pixel(center.into())? == &green);
        Ok(())
    }

    // Convert an image to grayscale by making use of parallel iterators
    #[test]
    fn cvt_grayscale() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/lichtenstein.png");

        let mut img = Image::<Rgba<u8>>::open(&path)?;
        img.par_pixels_mut().for_each(|pixel| {
            let (r, g, b, _) = (pixel.r, pixel.g, pixel.b, pixel.a);
            let l = 0.299f32 * r as f32 + 0.587f32 * g as f32 + 0.114f32 * b as f32;
            let l = l as u8;
            *pixel = Rgba {
                r: l,
                g: l,
                b: l,
                a: l,
            };
        });

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("cvt_grayscale")?;
        }

        Ok(())
    }

    // Draw a point out of bounds, should still pass, as shape may be partially visible
    #[test]
    fn draw_partially_out_of_bounds_shape() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/eye_orange.png");

        let mut img = Image::<Rgba<u8>>::open(&path)?;
        let center = img.dimensions();

        let green = Rgba {
            r: 0u8,
            g: 255u8,
            b: 0u8,
            a: 150u8,
        };

        let black = Rgba {
            r: 0u8,
            g: 0u8,
            b: 0u8,
            a: 0u8,
        };

        img.draw(Circle {
            position: center,
            color: green,
            radius: 100,
            filled: false,
            thickness: 5,
        })?;

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("draw_partially_out_of_bounds_shape")?;
        }

        assert!(img.get_pixel((center.0 - 1, center.1 - 1))? == &black);
        Ok(())
    }

    // Create a Luma image and convert it to RGBA8
    #[test]
    fn create_luma_image_and_convert() -> Result<()> {
        let mut img = Image::<Luma<u8>>::new(512, 512);
        img.par_pixels_mut().enumerate().for_each(|(idx, pixel)| {
            let x = idx % 512;
            let value = (x as f32 / 511.0 * 255.0) as u8;
            *pixel = Luma { l: value };
        });

        assert!(!img.is_empty());
        assert_eq!(img.dimensions(), (512, 512));
        assert_eq!(img.get_pixel((0, 0))?.l, 0);
        assert_eq!(img.get_pixel((511, 0))?.l, 255);
        if std::env::var("NO_DISPLAY").is_err() {
            img.display("create_luma_image_and_convert")?;
        }

        Ok(())
    }
}
