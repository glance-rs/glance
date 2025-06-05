use super::traits::Drawable;
use crate::{
    Result,
    img::{Image, pixel::Pixel},
};

/// A circle shape that can be drawn onto an image.
/// Can be either filled or drawn as an outline with a specified thickness.
/// The color is specified in RGBA8 format.
pub struct Circle<P: Pixel> {
    /// Center of the circle (x, y)
    pub position: (usize, usize),
    /// Color as a struct that implements Pixel (like [`Rgba`], [`Luma`])
    pub color: P,
    /// Radius in pixels
    pub radius: u32,
    /// Fill the shape (true) or draw outline (false)
    pub filled: bool,
    /// Outline thickness (only used when `filled = false`)
    pub thickness: u32,
}

impl<P> Drawable<P> for Circle<P>
where
    P: Pixel,
{
    fn draw_on(&self, image: &mut Image<P>) -> Result<()> {
        let (cx, cy) = (self.position.0 as i32, self.position.1 as i32);
        let radius = self.radius as i32;
        let thickness = self.thickness as i32;
        let dims = image.dimensions();

        let outer_radius = radius + thickness;
        let outer_radius_sq = outer_radius.pow(2);
        let inner_radius_sq = radius.pow(2);

        for dy in -outer_radius..outer_radius {
            for dx in -outer_radius..outer_radius {
                let nx = cx + dx;
                let ny = cy + dy;

                // Early skip for out of bounds pixels
                if nx < 0 || ny < 0 {
                    continue;
                }

                let (nx, ny) = (nx as usize, ny as usize);
                if nx >= dims.0 || ny >= dims.1 {
                    continue;
                }

                let distance_sq = dx * dx + dy * dy;
                // Check if (nx, ny) is within bounds
                let draw_pixel = match self.filled {
                    true => distance_sq <= inner_radius_sq,
                    false => distance_sq <= outer_radius_sq && distance_sq >= inner_radius_sq,
                };

                if draw_pixel {
                    image.set_pixel((nx, ny), self.color)?;
                }
            }
        }
        Ok(())
    }
}

/// An axis aligned bounding box that can be drawn onto an image.
/// Can be either filled or drawn as an outline with a specified thickness.
/// The color is specified in RGBA8 format.
pub struct AABB<P: Pixel> {
    /// Top-left corner (x, y)
    pub position: (usize, usize),
    /// Size as [width, height]
    pub size: (usize, usize),
    /// Color in RGBA8 format
    pub color: P,
    /// Fill the shape (true) or draw outline (false)
    pub filled: bool,
    /// Outline thickness (only used when `filled = false`)
    pub thickness: u32,
}

impl<P> Drawable<P> for AABB<P>
where
    P: Pixel,
{
    fn draw_on(&self, image: &mut Image<P>) -> Result<()> {
        let (cx, cy) = (self.position.0 as i32, self.position.1 as i32);
        let dims = image.dimensions();
        let width = self.size.0 as i32;
        let height = self.size.1 as i32;
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

                let nx = x as usize;
                let ny = y as usize;

                if nx >= dims.0 || ny >= dims.1 {
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
                    image.set_pixel((nx, ny), self.color)?;
                }
            }
        }

        Ok(())
    }
}

/// A line that can be drawn onto an image. Uses a square brush (Bresenham's algorithm).
/// The color is specified in RGBA8 format.
pub struct Line<P: Pixel> {
    /// Start point (x, y)
    pub start: (usize, usize),
    /// End point (x, y)
    pub end: (usize, usize),
    /// Color in RGBA8 format
    pub color: P,
    /// Line segment thickness (only used when `filled = false`)
    pub thickness: u32,
}

impl<P> Drawable<P> for Line<P>
where
    P: Pixel,
{
    fn draw_on(&self, image: &mut Image<P>) -> Result<()> {
        let (x0, y0) = self.start;
        let (x1, y1) = self.end;

        let dims = image.dimensions();

        let x0 = x0 as i32;
        let y0 = y0 as i32;
        let x1 = x1 as i32;
        let y1 = y1 as i32;
        let thickness = self.thickness as i32;
        let half_thickness = thickness / 2;

        // Difference in each direction
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        // Step in each direcion
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            for tx in -half_thickness..=half_thickness {
                for ty in -half_thickness..=half_thickness {
                    let nx = x + tx;
                    let ny = y + ty;

                    // Early return for out of bounds pixels
                    if nx < 0 || ny < 0 {
                        continue;
                    }

                    let (nx, ny) = (nx as usize, ny as usize);
                    if nx >= dims.0 || ny >= dims.1 {
                        continue;
                    }

                    image.set_pixel((nx, ny), self.color)?;
                }
            }

            // Done if reached end point
            if x == x1 && y == y1 {
                break;
            }

            let err_twice = 2 * err;
            if err_twice >= dy {
                err += dy;
                x += sx;
            }
            if err_twice <= dx {
                err += dx;
                y += sy;
            }
        }

        Ok(())
    }
}
