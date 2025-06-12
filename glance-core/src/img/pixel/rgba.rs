use super::Pixel;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Pixel for Rgba {
    fn channel_count() -> usize {
        4
    }

    fn new() -> Self {
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }

    fn from_rgba8(rgba: [u8; 4]) -> Self {
        Rgba {
            r: rgba[0] as f32 / 255.0,
            g: rgba[1] as f32 / 255.0,
            b: rgba[2] as f32 / 255.0,
            a: rgba[3] as f32 / 255.0,
        }
    }
    fn to_rgba8(&self) -> [u8; 4] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        ]
    }
}

impl From<[u8; 4]> for Rgba {
    fn from(value: [u8; 4]) -> Self {
        Rgba {
            r: value[0] as f32 / 255.0,
            g: value[1] as f32 / 255.0,
            b: value[2] as f32 / 255.0,
            a: value[3] as f32 / 255.0,
        }
    }
}

impl Rgba {
    pub fn apply<F>(mut self, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        self.r = f(self.r);
        self.g = f(self.g);
        self.b = f(self.b);
        self.a = f(self.a);
        self
    }
}
