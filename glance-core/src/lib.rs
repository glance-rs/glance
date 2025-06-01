pub mod drawing;
mod error;
pub mod img;
pub(crate) mod utils;

pub use self::error::{CoreError, Result};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawing::shapes::{AABB, Circle},
        img::Image,
    };
    use std::path::PathBuf;

    // Test with a real image file
    #[test]
    fn open_valid_image() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/eye.png");

        let img = Image::open(&path)?;

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("open_valid_image")?;
        }

        assert!(!img.is_empty());
        Ok(())
    }

    // Test error case for missing file
    #[test]
    fn open_invalid_path() {
        let result = Image::open("non_existent_file.jpg");
        assert!(result.is_err());
    }

    // Draw a point in the center of an image
    #[test]
    fn draw_shapes() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/eye_orange.png");

        let mut img = Image::open(&path)?;
        let dims = img.dimensions();

        let center = [dims[0] / 2, dims[1] / 2];
        let green = [0, 255, 0, 150];
        let blue = [0, 0, 255, 155];

        img.draw(Circle {
            position: center,
            color: green,
            radius: 100,
            filled: true,
            thickness: 5,
        })?;

        img.draw(AABB {
            position: [center[0] - 100, center[1] - 100],
            color: blue,
            size: [200, 200],
            thickness: 2,
            filled: false,
        })?;

        img.draw(Circle {
            position: center,
            color: blue,
            radius: 150,
            filled: false,
            thickness: 8,
        })?;

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("draw_shapes")?;
        }

        assert!(img.get_pixel(center)? == green);
        Ok(())
    }

    // Draw a point out of bounds, should still pass, as shape may be partially visible
    #[test]
    fn draw_partially_out_of_bounds_shape() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/eye_orange.png");

        let mut img = Image::open(&path)?;
        let dims = img.dimensions();

        let center = [dims[0], dims[1]];
        let green = [0, 255, 0, 255];

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

        assert!(img.get_pixel([dims[0] - 1, dims[1] - 1])? == [0, 0, 0, 0]);
        Ok(())
    }
}
