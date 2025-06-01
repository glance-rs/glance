use super::Image;

pub struct PixelIter<'a> {
    image: &'a Image,
    idx: usize,
}

impl<'a> Iterator for PixelIter<'a> {
    type Item = (usize, usize, [u8; 4]);
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= (self.image.width * self.image.height) as usize {
            return None;
        }
        let x = self.idx % self.image.width as usize;
        let y = self.idx / self.image.width as usize;
        let base = self.idx * 4;
        let px = [
            self.image.data[base],
            self.image.data[base + 1],
            self.image.data[base + 2],
            self.image.data[base + 3],
        ];
        self.idx += 1;
        Some((x, y, px))
    }
}

pub struct PixelIterMut<'a> {
    image: &'a mut Image,
    idx: usize,
}

impl<'a> Iterator for PixelIterMut<'a> {
    type Item = (usize, usize, &'a mut [u8; 4]);
    fn next(&mut self) -> Option<Self::Item> {
        let width = self.image.width as usize;
        let height = self.image.height as usize;
        if self.idx >= width * height {
            return None;
        }
        let x = self.idx % width;
        let y = self.idx / width;
        let base = self.idx * 4;

        // Each pixel here is only yielded once. So this is safe.
        let pixel = unsafe {
            let ptr = self.image.data.as_mut_ptr().add(base) as *mut [u8; 4];
            &mut *ptr
        };

        self.idx += 1;
        Some((x, y, pixel))
    }
}

impl Image {
    pub fn pixels(&self) -> PixelIter<'_> {
        PixelIter {
            image: self,
            idx: 0,
        }
    }

    pub fn pixels_mut(&mut self) -> PixelIterMut<'_> {
        PixelIterMut {
            image: self,
            idx: 0,
        }
    }
}
