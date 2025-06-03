use crate::Result;
use crate::img::Image;

/// Trait for anything that can be overlayed on top of an image.
pub trait Drawable {
    fn draw_on(&self, image: &mut Image) -> Result<()>;
}
