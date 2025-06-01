use rayon::iter::ParallelIterator;

#[derive(Debug, Clone, Copy)]
pub enum ThresholdType {
    Binary,
    Truncate,
    ToZero,
}

/// Extension trait for [`glance_core::img::Image`] to provide point operations
pub trait PointOpsExt {
    fn invert(self) -> Self;
    fn gamma(self, gamma: f32) -> Self;
    fn grayscale(self) -> Self;
    fn threshold(self, threshold: u8, max_intensity: u8, kind: ThresholdType) -> Self;
}

impl PointOpsExt for glance_core::img::Image {
    /// Returns an inverted image
    fn invert(mut self) -> Self {
        self.par_pixels_mut().for_each(|(_, _, pixel)| {
            pixel[0] = 255 - pixel[0];
            pixel[1] = 255 - pixel[1];
            pixel[2] = 255 - pixel[2];
        });

        self
    }

    /// Returns an image with given gamma applied.
    /// final = initial ^ (1 / gamma)
    fn gamma(mut self, gamma: f32) -> Self {
        let inv_gamma = 1.0 / gamma;

        self.pixels_mut().for_each(|(_, _, pixel)| {
            pixel[0] = ((pixel[0] as f32 / 255.0).powf(inv_gamma) * 255.0) as u8;
            pixel[1] = ((pixel[1] as f32 / 255.0).powf(inv_gamma) * 255.0) as u8;
            pixel[2] = ((pixel[2] as f32 / 255.0).powf(inv_gamma) * 255.0) as u8;
        });

        self
    }

    /// Returns a grayscaled image.
    ///
    /// Note: The image is still by all means an RGBA8 image.
    fn grayscale(mut self) -> Self {
        self.par_pixels_mut().for_each(|(_, _, pixel)| {
            let gray = ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as u8;
            pixel[0] = gray;
            pixel[1] = gray;
            pixel[2] = gray;
        });
        self
    }

    /// Returns the image with the given threshold operation applied.
    ///
    /// Note: This operation assumes the image has already been grayscaled and therefore only uses
    /// the red channel. Other channels are not considered.
    fn threshold(mut self, threshold: u8, max_intensity: u8, kind: ThresholdType) -> Self {
        self.par_pixels_mut().for_each(|(_, _, pixel)| {
            let intensity = pixel[0];
            let new_value = match kind {
                ThresholdType::Binary => {
                    if intensity > threshold {
                        max_intensity
                    } else {
                        0
                    }
                }
                ThresholdType::Truncate => {
                    if intensity > threshold {
                        threshold
                    } else {
                        intensity
                    }
                }
                ThresholdType::ToZero => {
                    if intensity > threshold {
                        0
                    } else {
                        intensity
                    }
                }
            };

            pixel[0] = new_value;
            pixel[1] = new_value;
            pixel[2] = new_value;
        });

        self
    }
}
