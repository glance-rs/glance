//! This module provides traits and types for working with different pixel formats
//! It assumes a base pixel format of RGBA8, and allows conversion to and from that format.
use std::ops::{Add, Div, Mul, Sub};

use crate::Result;

pub trait Pixel: PartialEq + Copy + Clone + Send + Sync + 'static {
    type Subpixel: Primitive + NumCast + AsPrimitive<f32> + Copy + 'static;

    fn channel_count() -> usize;
    fn channels(&self) -> Vec<Self::Subpixel>;
    fn from_rgba8(rgba: [u8; 4]) -> Result<Self>;
    fn to_rgba8(&self) -> [u8; 4];
}

pub trait Primitive:
    Copy
    + PartialEq
    + PartialOrd
    + Send
    + Sync
    + 'static
    + NumCast
    + AsPrimitive<f32>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    fn min_bound() -> f32;
    fn max_bound() -> f32;
}

impl Primitive for u8 {
    fn min_bound() -> f32 {
        0.0
    }
    fn max_bound() -> f32 {
        255.0
    }
}

impl Primitive for u16 {
    fn min_bound() -> f32 {
        0.0
    }
    fn max_bound() -> f32 {
        65535.0
    }
}

impl Primitive for f32 {
    fn min_bound() -> f32 {
        0.0
    }
    fn max_bound() -> f32 {
        1.0
    }
}

pub mod luma;
pub mod rgba;

pub use luma::*;
use num_traits::{AsPrimitive, NumCast};
pub use rgba::*;
