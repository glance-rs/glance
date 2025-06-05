use crate::Result;
use crate::img::Image;
use crate::img::pixel::Pixel;

/// Trait for anything that can be overlayed on top of an image.
pub trait Drawable<P: Pixel> {
    fn draw_on(&self, image: &mut Image<P>) -> Result<()>;
}
