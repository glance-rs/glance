use glance_core::img::{
    Image,
    pixel::{Luma, Primitive, Rgba},
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
pub trait PointOpsExtRgba<T: Primitive> {
    fn invert(self) -> Self;
    fn gamma(self, gamma: f32) -> Self;
    fn grayscale(self) -> Image<Luma<T>>;
    //fn histrogram_equalize(self) -> Self;
    fn lerp(self, other: &Image<Rgba<T>>, alpha: f32) -> Image<Rgba<T>>;
}

/// Extension trait for [`glance_core::img::Image`] to provide point operations for Luma images
pub trait PointOpsExtLuma<T: Primitive> {
    fn invert(self) -> Self;
    fn gamma(self, gamma: f32) -> Self;
    fn threshold(self, threshold: T, max_intensity: T, kind: ThresholdType) -> Image<Luma<T>>;
    fn histrogram_equalize(self) -> Self;
}

impl<T> PointOpsExtRgba<T> for Image<Rgba<T>>
where
    T: Primitive,
{
    fn invert(mut self) -> Self {
        let max_value = T::from(T::max_bound()).expect("Failed to convert max bound to T");
        self.par_pixels_mut().for_each(|pixel| {
            *pixel = Rgba {
                r: max_value - pixel.r,
                g: max_value - pixel.g,
                b: max_value - pixel.b,
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
            let r = T::from((pixel.r.as_() / T::max_bound()).powf(inv_gamma) * T::max_bound())
                .expect("Failed to convert gamma value to T");
            let g = T::from((pixel.g.as_() / T::max_bound()).powf(inv_gamma) * T::max_bound())
                .expect("Failed to convert gamma value to T");
            let b = T::from((pixel.b.as_() / T::max_bound()).powf(inv_gamma) * T::max_bound())
                .expect("Failed to convert gamma value to T");

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
    fn grayscale(self) -> Image<Luma<T>> {
        let (width, height) = self.dimensions();
        let gray_pixels = self
            .pixels()
            .map(|pixel| {
                let intensity =
                    (pixel.r.as_() * 0.299 + pixel.g.as_() * 0.587 + pixel.b.as_() * 0.114).round();
                let intensity = T::from(intensity).expect("Failed to convert intensity to T");
                Luma { l: intensity }
            })
            .collect::<Vec<_>>();

        Image::from_data(width, height, gray_pixels).unwrap()
    }

    fn lerp(self, other: &Image<Rgba<T>>, alpha: f32) -> Image<Rgba<T>> {
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
                r: T::from(px1.r.as_() * (1.0 - alpha) + px2.r.as_() * alpha)
                    .expect("Failed to convert lerped value to T"),
                g: T::from(px1.g.as_() * (1.0 - alpha) + px2.g.as_() * alpha)
                    .expect("Failed to convert lerped value to T"),
                b: T::from(px1.b.as_() * (1.0 - alpha) + px2.b.as_() * alpha)
                    .expect("Failed to convert lerped value to T"),
                a: T::from(T::max_bound()).expect("Failed to convert lerped value to T"),
            })
            .collect::<Vec<_>>();

        Image::from_data(width, height, lerped_pixels).unwrap()
    }
}

impl<T> PointOpsExtLuma<T> for Image<Luma<T>>
where
    T: Primitive,
{
    fn invert(mut self) -> Self {
        let max_value = T::from(T::max_bound()).expect("Failed to convert max bound to T");
        self.par_pixels_mut().for_each(|pixel| {
            *pixel = Luma {
                l: max_value - pixel.l,
            };
        });

        self
    }

    fn gamma(mut self, gamma: f32) -> Self {
        let inv_gamma = 1.0 / gamma;

        self.par_pixels_mut().for_each(|pixel| {
            let l = T::from((pixel.l.as_() / T::max_bound()).powf(inv_gamma) * T::max_bound())
                .expect("Failed to convert gamma value to T");

            *pixel = Luma { l };
        });

        self
    }

    /// Applies a threshold to the image, modifying pixel intensities based on the specified
    /// threshold type.
    /// Binary => Pixels above the threshold are set to `max_intensity`, others to 0.
    /// Truncate => Pixels above the threshold are set to the threshold value, others remain
    /// unchanged.
    /// ToZero => Pixels above the threshold remain unchanged, others are set to 0.
    fn threshold(self, threshold: T, max_intensity: T, kind: ThresholdType) -> Image<Luma<T>> {
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
                            T::from(0).unwrap()
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
                            T::from(0).unwrap()
                        }
                    }
                };
                Luma { l: new_l }
            })
            .collect::<Vec<_>>();

        Image::from_data(width, height, thresholded_pixels).unwrap()
    }

    /// Adaptive histrogram equalization for grayscaled images.
    /// Assumes luminance is in the red channel (in accordance with the [`PointOpsExt::grayscale`] function)
    fn histrogram_equalize(mut self) -> Self {
        let (width, height) = self.dimensions();
        let pixel_count = (width * height) as u32;
        let channel_max = T::max_bound() as usize;

        // Find histogram
        let mut hist = vec![0u32; channel_max + 1];
        self.pixels().for_each(|pixel| {
            let idx = pixel.l.as_() as usize;
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
        let mut lookup_table = vec![T::from(0).unwrap(); channel_max + 1];
        let scale = channel_max as f32 / (pixel_count - cdf_min) as f32;

        for (i, value) in cdf.iter().enumerate() {
            let adjusted =
                ((*value as f32 - cdf_min as f32) * scale).clamp(0.0, channel_max as f32);
            lookup_table[i] =
                T::from(adjusted.round()).expect("Failed to convert lookup value to T");
        }

        // Apply equalization
        self.par_pixels_mut().for_each(|pixel| {
            let intensity = pixel.l.as_() as usize;
            let new_intensity = lookup_table[intensity];
            pixel.l = T::from(new_intensity).expect("Failed to convert lookup value to T");
        });

        self
    }
}
