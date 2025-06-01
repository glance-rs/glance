pub mod drawing;
mod error;
pub mod img;
pub(crate) mod utils;

pub use self::error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{drawing::shapes::Circle, img::Image};
    use std::path::PathBuf;

    // Test with a real image file
    #[test]
    fn open_valid_image() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/eye.png");

        let img = Image::open(&path)?;
        assert!(!img.is_empty());
        Ok(())
    }

    // Test error case for missing file
    #[test]
    fn open_invalid_path() {
        let result = Image::open("non_existent_file.jpg");
        assert!(result.is_err());
    }

    // Convert to grayscale
    #[test]
    fn cvt_grayscale() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/eye_orange.png");

        let img = Image::open(&path)?.into_grayscale()?;
        assert!(!img.is_empty());
        Ok(())
    }

    // Draw a point in the center of an image
    #[test]
    fn draw_point() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/eye_orange.png");

        let mut img = Image::open(&path)?;
        let dims = img.dimensions();

        let center = [dims[0] / 2, dims[1] / 2];
        let green = [0, 255, 0, 150];

        img.draw(Circle {
            position: center,
            color: green,
            radius: 100,
        })?;

        assert!(img.get_pixel(center)? == green);
        Ok(())
    }

    // Draw a point out of bounds, should still pass, as shape may be partially visible
    #[test]
    fn draw_invalid_point() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/eye_orange.png");

        let mut img = Image::open(&path)?;
        let dims = img.dimensions();

        let center = [dims[0], dims[1]];
        let green = [0, 255, 0, 255];

        img.draw(Circle {
            position: center,
            color: green,
            radius: 1,
        })?;

        Ok(())
    }
}
