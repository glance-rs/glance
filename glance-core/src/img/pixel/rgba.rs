use num_traits::{AsPrimitive, NumCast};

use crate::{CoreError, Result};

use super::{Pixel, Primitive};

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Rgba<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Pixel for Rgba<T>
where
    T: Copy + Primitive + NumCast + AsPrimitive<f32> + PartialEq + Send + Sync + 'static,
{
    type Subpixel = T;
    fn channel_count() -> usize {
        4
    }

    fn channels(&self) -> Vec<Self::Subpixel> {
        vec![self.r, self.g, self.b, self.a]
    }

    fn from_rgba8(rgba: [u8; 4]) -> Result<Self> {
        // Scale factor from u8's range to T's range
        let scale = T::max_bound() / u8::max_bound();
        let convert_channel = |value: u8| -> Result<T> {
            let scaled = value as f32 * scale;
            T::from(scaled).ok_or_else(|| {
                CoreError::InvalidCast(format!("Failed to cast {} to target type", scaled))
            })
        };

        Ok(Rgba {
            r: convert_channel(rgba[0])?,
            g: convert_channel(rgba[1])?,
            b: convert_channel(rgba[2])?,
            a: convert_channel(rgba[3])?,
        })
    }

    fn to_rgba8(&self) -> [u8; 4] {
        // Scale factor from T's range to u8's range
        let scale = u8::max_bound() / T::max_bound();

        // Helper function to convert each channel
        let convert_channel = |value: T| -> u8 {
            let scaled = value.as_() * scale;
            scaled.round() as u8
        };

        [
            convert_channel(self.r),
            convert_channel(self.g),
            convert_channel(self.b),
            convert_channel(self.a),
        ]
    }
}
