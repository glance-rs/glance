use super::traits::Drawable;
use crate::Result;

/// A circle shape that can be drawn onto an image.
/// Can be either filled or drawn as an outline with a specified thickness.
/// The color is specified in RGBA8 format.
pub struct Circle {
    /// Center of the circle (x, y)
    pub position: [u32; 2],
    /// Color in RGBA8 format
    pub color: [u8; 4],
    /// Radius in pixels
    pub radius: u32,
    /// Fill the shape (true) or draw outline (false)
    pub filled: bool,
    /// Outline thickness (only used when `filled = false`)
    pub thickness: u32,
}

impl Drawable for Circle {
    fn draw_on(&self, image: &mut crate::img::Image) -> Result<()> {
        let (cx, cy) = (self.position[0] as i32, self.position[1] as i32);
        let radius = self.radius as i32;
        let thickness = self.thickness as i32;
        let dims = image.dimensions();

        let outer_radius_sq = radius + thickness;
        let inner_radius = radius.pow(2);

        for dy in -radius..radius {
            for dx in -radius..radius {
                let nx = cx + dx;
                let ny = cy + dy;

                // Early skip for out of bounds pixels
                if nx < 0 || ny < 0 {
                    continue;
                }

                let (nx, ny) = (nx as u32, ny as u32);
                if nx >= dims[0] || ny >= dims[1] {
                    continue;
                }

                let distance_sq = dx * dx + dy * dy;
                // Check if (nx, ny) is within bounds
                let draw_pixel = match self.filled {
                    true => distance_sq <= outer_radius_sq,
                    false => distance_sq <= outer_radius_sq && distance_sq >= inner_radius,
                };

                if draw_pixel {
                    image.alpha_blend_pixel([nx, ny], self.color)?;
                }
            }
        }
        Ok(())
    }
}

/// An axis aligned bounding box that can be drawn onto an image.
/// Can be either filled or drawn as an outline with a specified thickness.
/// The color is specified in RGBA8 format.
pub struct AABB {
    /// Top-left corner (x, y)
    pub position: [u32; 2],
    /// Size as [width, height]
    pub size: [u32; 2],
    /// Color in RGBA8 format
    pub color: [u8; 4],
    /// Fill the shape (true) or draw outline (false)
    pub filled: bool,
    /// Outline thickness (only used when `filled = false`)
    pub thickness: u32,
}

impl Drawable for AABB {
    fn draw_on(&self, image: &mut crate::img::Image) -> Result<()> {
        let (cx, cy) = (self.position[0] as i32, self.position[1] as i32);
        let dims = image.dimensions();
        let width = self.size[0] as i32;
        let height = self.size[1] as i32;
        let thickness = self.thickness as i32;

        let left_x = cx - thickness;
        let right_x = cx + width + thickness;
        let top_y = cy - thickness;
        let bottom_y = cy + height + thickness;

        // Go through all pixels inside the rectangle
        for x in left_x..right_x {
            for y in top_y..bottom_y {
                // Early return for out of bounds pixels
                if x < 0 || y < 0 {
                    continue;
                }

                let nx = x as u32;
                let ny = y as u32;

                if nx >= dims[0] || ny >= dims[1] {
                    continue;
                }

                // If not filled, only draw outline
                let on_left_edge = x - left_x <= thickness;
                let on_right_edge = right_x - x <= thickness;
                let on_top_edge = y - top_y < thickness;
                let on_bottom_edge = bottom_y - y <= thickness;
                let draw_pixel =
                    self.filled || on_left_edge || on_right_edge || on_top_edge || on_bottom_edge;

                if draw_pixel {
                    image.alpha_blend_pixel([nx, ny], self.color)?;
                }
            }
        }

        Ok(())
    }
}
