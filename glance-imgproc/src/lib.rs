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

        let img = Image::<Rgba<u8>>::open(&path)?;
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

        let img = Image::<Rgba<u16>>::open(&path)?;
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

        let img = Image::<Rgba<u16>>::open(&path)?;
        let img = img
            .grayscale()
            .threshold(32000, 65535, point_ops::ThresholdType::Binary);

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("threshold_image")?;
        }

        Ok(())
    }

    #[test]
    fn hist_equalize_luma_image() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/lichtenstein.png");

        let img = Image::<Rgba<u16>>::open(&path)?;
        let img = img.grayscale().histrogram_equalize();

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("hist_equalize_luma_image")?;
        }

        Ok(())
    }
}
