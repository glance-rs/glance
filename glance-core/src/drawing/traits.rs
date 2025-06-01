use crate::Result;
use crate::img::Image;

pub trait Drawable {
    fn draw_on(&self, image: &mut Image) -> Result<()>;
}
