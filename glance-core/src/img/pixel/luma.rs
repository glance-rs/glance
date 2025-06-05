use num_traits::{AsPrimitive, NumCast};

use crate::{CoreError, Result};

use super::{Pixel, Primitive};

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Luma<T> {
    pub l: T,
}

impl<T> Pixel for Luma<T>
where
    T: Copy + Primitive + NumCast + AsPrimitive<f32> + PartialEq + Send + Sync + 'static,
{
    type Subpixel = T;
    fn channel_count() -> usize {
        1
    }

    fn channels(&self) -> Vec<Self::Subpixel> {
        vec![self.l]
    }

    fn from_rgba8(rgba: [u8; 4]) -> Result<Self> {
        let scale = T::max_bound() / u8::max_bound();
        let luma =
            (0.299f32 * rgba[0] as f32 + 0.587f32 * rgba[1] as f32 + 0.114f32 * rgba[2] as f32)
                * scale;
        let l = T::from(luma).ok_or_else(|| {
            CoreError::InvalidCast(format!("Failed to cast {} to target type", luma))
        })?;

        Ok(Luma { l })
    }

    fn to_rgba8(&self) -> [u8; 4] {
        // Scale factor from T range to u8 range
        let scale = u8::max_bound() / T::max_bound();

        // Helper function to convert each channel
        let convert_channel = |value: T| -> u8 {
            let scaled = value.as_() * scale;
            scaled.round() as u8
        };

        let l = convert_channel(self.l);

        [l, l, l, 255]
    }
}
