use glance_core::img::{
    Image,
    pixel::{Luma, Rgba},
};
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone, Copy)]
pub enum ThresholdType {
    /// Pixels above the threshold are set to `max_intensity`, others to 0.
    Binary,
    /// Pixels above the threshold are set to the threshold value, others remain unchanged.
    Truncate,
    /// Pixels above the threshold remain unchanged, others are set to 0.
    ToZero,
}

/// Extension trait for [`glance_core::img::Image`] to provide point operations for RGBA images
pub trait PointOpsExtRgba {
    fn invert(self) -> Self;
    fn gamma(self, gamma: f32) -> Self;
    fn grayscale(self) -> Image<Luma>;
    //fn histrogram_equalize(self) -> Self;
    fn lerp(self, other: &Image<Rgba>, alpha: f32) -> Image<Rgba>;
    fn brightness(self, brightness: f32) -> Image<Rgba>;
    fn contrast(self, contrast: f32) -> Image<Rgba>;
}

/// Extension trait for [`glance_core::img::Image`] to provide point operations for Luma images
pub trait PointOpsExtLuma {
    fn invert(self) -> Self;
    fn gamma(self, gamma: f32) -> Self;
    fn lerp(self, other: &Image<Luma>, alpha: f32) -> Image<Luma>;
    fn threshold(self, threshold: f32, max_intensity: f32, kind: ThresholdType) -> Image<Luma>;
    fn histrogram_equalize(self) -> Self;
}

impl PointOpsExtRgba for Image<Rgba> {
    /// Inverts the colors of the image by subtracting each pixel's RGB values from the maximum value
    fn invert(mut self) -> Self {
        self.par_pixels_mut().for_each(|pixel| {
            *pixel = Rgba {
                r: 1.0 - pixel.r,
                g: 1.0 - pixel.g,
                b: 1.0 - pixel.b,
                a: pixel.a, // Preserve alpha channel
            };
        });

        self
    }

    /// Returns an image with given gamma applied.
    /// final = initial ^ (1 / gamma)
    fn gamma(mut self, gamma: f32) -> Self {
        let inv_gamma = 1.0 / gamma;

        self.par_pixels_mut().for_each(|pixel| {
            let r = pixel.r.powf(inv_gamma);
            let g = pixel.g.powf(inv_gamma);
            let b = pixel.b.powf(inv_gamma);
            *pixel = Rgba {
                r,
                g,
                b,
                a: pixel.a, // Preserve alpha channel
            };
        });

        self
    }

    /// Returns a grayscale image from the RGBA image. Weights are in accordance with the BT.601
    /// standard. The returned image maintains the prcision of the original image's pixel type, but with only
    /// one channel (luminance) (see [`Luma`]).
    fn grayscale(self) -> Image<Luma> {
        let (width, height) = self.dimensions();
        let gray_pixels = self
            .pixels()
            .map(|pixel| {
                let intensity = pixel.r * 0.299 + pixel.g * 0.587 + pixel.b * 0.114;
                Luma { l: intensity }
            })
            .collect();

        Image::from_data(width, height, gray_pixels).unwrap()
    }

    /// Linearly interpolates between two images of the same dimensions.
    /// The alpha parameter controls the interpolation factor.
    fn lerp(self, other: &Image<Rgba>, alpha: f32) -> Image<Rgba> {
        let (width, height) = self.dimensions();
        if (width, height) != other.dimensions() {
            panic!(
                "Cannot lerp images of different dimensions: {:?} and {:?}",
                (width, height),
                other.dimensions()
            );
        }
        let lerped_pixels = self
            .pixels()
            .zip(other.pixels())
            .map(|(px1, px2)| Rgba {
                r: px1.r * (1.0 - alpha) + px2.r * alpha,
                g: px1.g * (1.0 - alpha) + px2.g * alpha,
                b: px1.b * (1.0 - alpha) + px2.b * alpha,
                a: px1.a * (1.0 - alpha) + px2.a * alpha,
            })
            .collect::<Vec<_>>();

        Image::from_data(width, height, lerped_pixels).unwrap()
    }

    /// Adjusts the brightness of the image by adding a value to each pixel's RGB channels.
    /// The intensities are clamped to the [0.0, 1.0] range.
    fn brightness(self, brightness: f32) -> Image<Rgba> {
        let (width, height) = self.dimensions();
        let adjusted_pixels = self
            .pixels()
            .map(|pixel| Rgba {
                r: (pixel.r + brightness).clamp(0.0, 1.0),
                g: (pixel.g + brightness).clamp(0.0, 1.0),
                b: (pixel.b + brightness).clamp(0.0, 1.0),
                a: pixel.a,
            })
            .collect::<Vec<_>>();

        Image::from_data(width, height, adjusted_pixels).unwrap()
    }

    /// Adjusts the contrast of the image by multiplying each pixel's RGB channels by a value.
    /// The intensities are clamped to the [0.0, 1.0] range.
    fn contrast(self, contrast: f32) -> Image<Rgba> {
        let (width, height) = self.dimensions();
        let adjusted_pixels = self
            .pixels()
            .map(|pixel| Rgba {
                r: (pixel.r * contrast).clamp(0.0, 1.0),
                g: (pixel.g * contrast).clamp(0.0, 1.0),
                b: (pixel.b * contrast).clamp(0.0, 1.0),
                a: pixel.a, // Preserve alpha channel
            })
            .collect::<Vec<_>>();

        Image::from_data(width, height, adjusted_pixels).unwrap()
    }
}

impl PointOpsExtLuma for Image<Luma> {
    /// Inverts the colors of the image by subtracting each pixel's RGB values from the maximum value
    fn invert(mut self) -> Self {
        self.par_pixels_mut().for_each(|pixel| {
            *pixel = Luma { l: 1.0 - pixel.l };
        });

        self
    }

    /// Returns an image with given gamma applied.
    fn gamma(mut self, gamma: f32) -> Self {
        let inv_gamma = 1.0 / gamma;

        self.par_pixels_mut().for_each(|pixel| {
            *pixel = Luma {
                l: pixel.l.powf(inv_gamma),
            };
        });

        self
    }

    /// Applies a threshold to the image, modifying pixel intensities based on the specified
    /// threshold type.
    /// Binary => Pixels above the threshold are set to `max_intensity`, others to 0.
    /// Truncate => Pixels above the threshold are set to the threshold value, others remain
    /// unchanged.
    /// ToZero => Pixels above the threshold remain unchanged, others are set to 0.
    fn threshold(self, threshold: f32, max_intensity: f32, kind: ThresholdType) -> Image<Luma> {
        let (width, height) = self.dimensions();
        let thresholded_pixels = self
            .pixels()
            .map(|pixel| {
                let l = pixel.l;
                let new_l = match kind {
                    ThresholdType::Binary => {
                        if l >= threshold {
                            max_intensity
                        } else {
                            0.0
                        }
                    }
                    ThresholdType::Truncate => {
                        if l > threshold {
                            threshold
                        } else {
                            l
                        }
                    }
                    ThresholdType::ToZero => {
                        if l > threshold {
                            l
                        } else {
                            0.0
                        }
                    }
                };
                Luma { l: new_l }
            })
            .collect();

        Image::from_data(width, height, thresholded_pixels).unwrap()
    }

    /// Adaptive histrogram equalization for grayscaled images.
    /// Assumes luminance is in the red channel (in accordance with the [`PointOpsExt::grayscale`] function)
    fn histrogram_equalize(mut self) -> Self {
        let (width, height) = self.dimensions();
        let pixel_count = (width * height) as u32;
        let channel_max = 255_usize;

        // Find histogram
        let mut hist = vec![0u32; channel_max + 1];
        self.pixels().for_each(|pixel| {
            let idx = (pixel.l * 255.0).round() as usize;
            hist[idx] += 1;
        });

        // Calculate CDF
        let mut cdf = vec![0u32; channel_max + 1];
        cdf[0] = hist[0];
        for i in 1..channel_max + 1 {
            cdf[i] = cdf[i - 1] + hist[i];
        }

        // Find min
        let cdf_min = *cdf.iter().find(|&&x| x > 0).unwrap_or(&0);

        // Populate lookup table
        let mut lookup_table = vec![0.0; channel_max + 1];
        let scale = channel_max as f32 / (pixel_count - cdf_min) as f32;

        for (i, value) in cdf.iter().enumerate() {
            let adjusted =
                ((*value as f32 - cdf_min as f32) * scale).clamp(0.0, channel_max as f32);
            lookup_table[i] = adjusted;
        }

        // Apply equalization
        self.par_pixels_mut().for_each(|pixel| {
            let intensity = (pixel.l * 255.0).round() as usize;
            let new_intensity = lookup_table[intensity];
            pixel.l = new_intensity / 255.0; // Normalize back to [0.0, 1.0]
        });

        self
    }

    /// Linearly interpolates between two images of the same dimensions.
    /// The alpha parameter controls the interpolation factor.
    fn lerp(self, other: &Image<Luma>, alpha: f32) -> Image<Luma> {
        let (width, height) = self.dimensions();
        if (width, height) != other.dimensions() {
            panic!(
                "Cannot lerp images of different dimensions: {:?} and {:?}",
                (width, height),
                other.dimensions()
            );
        }
        let lerped_pixels = self
            .pixels()
            .zip(other.pixels())
            .map(|(px1, px2)| Luma {
                l: px1.l * (1.0 - alpha) + px2.l * alpha,
            })
            .collect::<Vec<_>>();

        Image::from_data(width, height, lerped_pixels).unwrap()
    }
}
