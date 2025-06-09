//! This module provides traits and types for working with different pixel formats
//! It assumes a base pixel format of RGBA8, and allows conversion to and from that format.

pub trait Pixel: PartialEq + Copy + Clone + Send + Sync + 'static {
    fn channel_count() -> usize;
    fn new() -> Self;
    fn from_rgba8(rgba: [u8; 4]) -> Self;
    fn to_rgba8(&self) -> [u8; 4];
}

pub mod luma;
pub mod rgba;

pub use luma::*;
pub use rgba::*;
