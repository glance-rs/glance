mod error;
pub mod point_ops;

pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::Result;
    use glance_core::img::Image;
    use glance_core::img::pixel::Rgba;

    use crate::point_ops::{PointOpsExtLuma, PointOpsExtRgba};

    use super::*;

    #[test]
    fn invert_image() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/flower.jpg");

        let img = Image::<Rgba>::open(&path)?;
        let img = img.invert();

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("invert_image")?;
        }

        Ok(())
    }

    #[test]
    fn grayscale_image() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/lichtenstein.png");

        let img = Image::<Rgba>::open(&path)?;
        let img = img.grayscale();

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("grayscale_image")?;
        }

        Ok(())
    }

    #[test]
    fn threshold_image() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/lichtenstein.png");

        let img = Image::<Rgba>::open(&path)?;
        let img = img
            .grayscale()
            .threshold(0.5, 1.0, point_ops::ThresholdType::Binary);

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("threshold_image")?;
        }

        Ok(())
    }

    #[test]
    fn hist_equalize_luma_image() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/lichtenstein.png");

        let img = Image::<Rgba>::open(&path)?;
        let img = img.grayscale().histrogram_equalize();

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("hist_equalize_luma_image")?;
        }

        Ok(())
    }

    #[test]
    fn lerp_images() -> Result<()> {
        let mut dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir_path.push("../media/test_imgs/");
        let path1 = dir_path.join("lichtenstein.png");
        let path2 = dir_path.join("pepper.bmp");

        let img1 = Image::<Rgba>::open(path1)?;
        let img2 = Image::<Rgba>::open(path2)?;

        let lerp_img = img1.lerp(&img2, 0.5);

        if std::env::var("NO_DISPLAY").is_err() {
            lerp_img.display("lerp_images")?;
        }

        Ok(())
    }

    #[test]
    fn brightness_contrast() -> Result<()> {
        let mut dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir_path.push("../media/test_imgs/");
        let path1 = dir_path.join("lichtenstein.png");

        let img1 = Image::<Rgba>::open(path1)?;
        let img1 = img1.contrast(1.9);

        if std::env::var("NO_DISPLAY").is_err() {
            img1.display("brightness_contrast")?;
        }

        Ok(())
    }
}
