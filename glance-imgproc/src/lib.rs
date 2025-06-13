pub mod affine_transformations;
pub mod kernels;
pub mod linear_filters;
pub mod nonlinear_filters;
pub mod point_ops;

mod error;
pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::Result;
    use crate::affine_transformations::AffineTransformationsExtLuma;
    use crate::kernels;
    use crate::linear_filters::{BorderMode, ConvolutionExtLuma};
    use crate::nonlinear_filters::NonLinearFilterExtLuma;
    use glance_core::img::pixel::Rgba;
    use glance_core::img::{Image, pixel::Luma};

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

    #[test]
    fn sobel_convolve() -> Result<()> {
        let mut dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir_path.push("../media/test_imgs/");
        let path = dir_path.join("lenna.png");

        let img = Image::<Luma>::open(path)?;
        let horizontal_kernel = kernels::sobel_x();
        let vertical_kernel = kernels::sobel_y();

        let horizontal_img = img
            .clone()
            .convolve_2d(horizontal_kernel, BorderMode::Replicate)
            .apply(|f| f32::powf(f, 2.0));
        let vertical_img = img
            .convolve_2d(vertical_kernel, BorderMode::Replicate)
            .apply(|f| f32::powf(f, 2.0));

        let img = horizontal_img
            .lerp(&vertical_img, 0.5)
            .apply(f32::sqrt)
            .normalize();

        if std::env::var("NO_DISPLAY").is_err() {
            img.display("sobel_convolve")?;
        }

        Ok(())
    }

    #[test]
    fn laplacian_convolve() -> Result<()> {
        let mut dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir_path.push("../media/test_imgs/");
        let path = dir_path.join("lenna.png");

        let img = Image::<Luma>::open(path)?;
        let laplacian_kernel = kernels::laplacian_3x3();
        let gaussian_kernel = kernels::gaussian_filter(3, 0.1);

        if std::env::var("NO_DISPLAY").is_err() {
            img.convolve_2d(gaussian_kernel, BorderMode::Replicate)
                .convolve_2d(laplacian_kernel, BorderMode::Replicate)
                .apply(f32::abs)
                .normalize()
                .display("laplacian_convolve")?;
        }

        Ok(())
    }

    #[test]
    fn nonlinear_filters() -> Result<()> {
        let mut dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir_path.push("../media/test_imgs/");
        let path = dir_path.join("noisy.png");

        let img = Image::<Luma>::open(path)?;
        let median_blurred_img = img.clone().median_blur(5);

        if std::env::var("NO_DISPLAY").is_err() {
            img.vstack(&median_blurred_img)?
                .display("nonlinear_filters_median_blur")?;
        }

        Ok(())
    }

    #[test]
    fn affine_transformations() -> Result<()> {
        let mut dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir_path.push("../media/test_imgs/");
        let path = dir_path.join("lenna.png");

        let img = Image::<Luma>::open(path)?;

        if std::env::var("NO_DISPLAY").is_err() {
            img.scale((2.0, 2.0))
                .translate((-50.0, -50.0))
                .display("affine_transformations_rotate")?;
        }

        Ok(())
    }
}
