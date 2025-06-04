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
    fn histrogram_equalize(self) -> Self;
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

        self.par_pixels_mut().for_each(|(_, _, pixel)| {
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
            let gray = (pixel[0] as f32 * 0.299 + pixel[1] as f32 * 0.587 + pixel[2] as f32 * 0.114)
                .round() as u8;
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

    /// Adaptive histrogram equalization for grayscaled images.
    /// Assumes luminance is in the red channel (in accordance with the [`PointOpsExt::grayscale`] function)
    fn histrogram_equalize(mut self) -> Self {
        let [width, height] = self.dimensions();
        let pixel_count = width * height;

        // Find histogram
        let mut hist = [0u32; 256];
        self.pixels().into_iter().for_each(|(_, _, pixel)| {
            hist[pixel[0] as usize] += 1;
        });

        // Calculate CDF
        let mut cdf = [0u32; 256];
        cdf[0] = hist[0];
        for i in 1..256 {
            cdf[i] = cdf[i - 1] + hist[i];
        }

        // Find min
        let cdf_min = *cdf.iter().find(|&&x| x > 0).unwrap_or(&0);

        // Populate lookup tabl
        let mut lookup_table = [0u8; 256];
        let scale = 255.0 / (pixel_count - cdf_min) as f32;

        for (i, value) in cdf.iter().enumerate() {
            let adjusted = ((*value as f32 - cdf_min as f32) * scale).clamp(0.0, 255.0);
            lookup_table[i] = adjusted.round() as u8;
        }

        // Apply equalization
        self.par_pixels_mut().for_each(|(_, _, pixel)| {
            let intensity = pixel[0];
            let new_intensity = lookup_table[intensity as usize];
            pixel[0] = new_intensity;
            pixel[1] = new_intensity;
            pixel[2] = new_intensity;
        });

        self
    }
}
