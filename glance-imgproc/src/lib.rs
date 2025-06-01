mod error;
pub mod point_ops;

pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::Result;
    use glance_core::img::Image;

    use crate::point_ops::PointOpsExt;

    use super::*;

    #[test]
    fn invert_image() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/flower.jpg");

        let img = Image::open(&path)?;
        let img = img.invert();

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("invert_image")?;
        }

        Ok(())
    }

    #[test]
    fn threshold_image() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../media/test_imgs/lichtenstein.png");

        let img = Image::open(&path).unwrap();
        let img = img
            .grayscale()
            .threshold(120, 255, point_ops::ThresholdType::Binary)
            .invert();

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("threshold_image")?;
        }

        Ok(())
    }
}
