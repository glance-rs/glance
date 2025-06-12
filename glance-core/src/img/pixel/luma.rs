use super::Pixel;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Luma {
    pub l: f32,
}

impl Pixel for Luma {
    fn channel_count() -> usize {
        1
    }

    fn new() -> Self {
        Luma { l: 0.0 }
    }

    fn from_rgba8(rgba: [u8; 4]) -> Self {
        Luma {
            l: (0.299f32 * rgba[0] as f32 + 0.587f32 * rgba[1] as f32 + 0.114f32 * rgba[2] as f32)
                / 255.0,
        }
    }

    fn to_rgba8(&self) -> [u8; 4] {
        let l = (self.l * 255.0).round() as u8;
        [l, l, l, 255]
    }
}

impl Luma {
    pub fn apply<F>(mut self, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        self.l = f(self.l);
        self
    }
}
