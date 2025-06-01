mod error;
pub mod img;

pub use self::error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::img::Image;
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
}
