use super::{Image, pixel::Pixel};
use rayon::prelude::*;

pub struct PixelIter<'a, P: Pixel> {
    iter: std::slice::Iter<'a, P>,
}

impl<'a, P: Pixel> PixelIter<'a, P> {
    pub fn new(image: &'a Image<P>) -> Self {
        Self {
            iter: image.data.iter(),
        }
    }
}

impl<'a, P: Pixel> Iterator for PixelIter<'a, P> {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().copied()
    }
}

pub struct PixelIterMut<'a, P: Pixel> {
    iter: std::slice::IterMut<'a, P>,
}

impl<'a, P: Pixel> PixelIterMut<'a, P> {
    pub fn new(image: &'a mut Image<P>) -> Self {
        Self {
            iter: image.data.iter_mut(),
        }
    }
}

impl<'a, P: Pixel> Iterator for PixelIterMut<'a, P> {
    type Item = &'a mut P;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<P> Image<P>
where
    P: Pixel,
{
    pub fn pixels(&self) -> PixelIter<P> {
        PixelIter::new(self)
    }

    pub fn pixels_mut(&mut self) -> PixelIterMut<P> {
        PixelIterMut::new(self)
    }

    pub fn par_pixels(&self) -> rayon::slice::Iter<'_, P> {
        self.data.par_iter()
    }

    pub fn par_pixels_mut(&mut self) -> rayon::slice::IterMut<'_, P> {
        self.data.par_iter_mut()
    }
}
