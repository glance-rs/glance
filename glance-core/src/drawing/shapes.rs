use super::traits::Drawable;
use crate::Result;

pub struct Circle {
    pub position: [u32; 2],
    pub color: [u8; 4],
    pub radius: u32,
}

impl Drawable for Circle {
    fn draw_on(&self, image: &mut crate::img::Image) -> Result<()> {
        let (cx, cy) = (self.position[0] as i32, self.position[1] as i32);
        let radius = self.radius as i32;
        let dims = image.dimensions();

        for dy in -radius..radius {
            for dx in -radius..radius {
                let nx = cx + dx;
                let ny = cy + dy;

                // Check if (nx, ny) is within bounds
                if (dx * dx + dy * dy) as f32 <= (radius as f32).powi(2) && nx >= 0 && ny >= 0 {
                    let (nx, ny) = (nx as u32, ny as u32);
                    if nx < dims[0] && ny < dims[1] {
                        image.alpha_blend_pixel([nx, ny], self.color)?;
                    }
                }
            }
        }
        Ok(())
    }
}
