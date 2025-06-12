use glance_core::img::{Image, pixel::Luma};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};

pub trait NonLinearFilterExtLuma {
    // Non-linear filters
    fn median_blur(self, kernel_size: usize) -> Image<Luma>;
    // Morphological operations
    fn dilate(self, kernel: &Image<Luma>) -> Image<Luma>;
    fn erode(self, kernel: &Image<Luma>) -> Image<Luma>;
}

impl NonLinearFilterExtLuma for Image<Luma> {
    /// Applies a median blur to the image using a square kernel of the given size.
    fn median_blur(self, kernel_size: usize) -> Image<Luma> {
        if kernel_size % 2 == 0 {
            panic!("Kernel size must be odd");
        }
        let (width, height) = self.dimensions();
        if width < kernel_size || height < kernel_size {
            panic!("Input image must be larger than the kernel");
        }

        let half_kernel = kernel_size / 2;
        let half_kernel = half_kernel as isize;

        let new_pixels = self
            .par_pixels()
            .enumerate()
            .map(|(idx, _pixel)| {
                let (x, y) = (idx % width, idx / width);
                let mut values: Vec<f32> = Vec::with_capacity(kernel_size * kernel_size);

                for ky in -half_kernel..=half_kernel {
                    for kx in -half_kernel..=half_kernel {
                        let input_x = x as isize + kx;
                        let input_y = y as isize + ky;

                        if input_x < 0
                            || input_y < 0
                            || input_x >= width as isize
                            || input_y >= height as isize
                        {
                            continue; // Skip out-of-bounds pixels
                        }

                        let input_pixel = self
                            .get_pixel((input_x as usize, input_y as usize))
                            .unwrap();
                        values.push(input_pixel.l);
                    }
                }

                values.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let median_value = values[values.len() / 2];
                Luma { l: median_value }
            })
            .collect();

        Image::from_data(width, height, new_pixels).unwrap()
    }

    fn dilate(self, kernel: &Image<Luma>) -> Image<Luma> {
        let (kernel_width, kernel_height) = kernel.dimensions();
        if kernel_width % 2 == 0 || kernel_height % 2 == 0 {
            panic!("Kernel size must be odd in both dimensions");
        }
        let (input_width, input_height) = self.dimensions();
        if input_width < kernel_width || input_height < kernel_height {
            panic!("Input image must be larger than the kernel");
        }

        let kernel_half_width = kernel_width / 2;
        let kernel_half_height = kernel_height / 2;

        let new_pixels = self
            .par_pixels()
            .enumerate()
            .map(|(idx, _pixel)| {
                let mut max_value: f32 = 0.0;
                let (x, y) = (idx % input_width, idx / input_width);

                for ky in 0..kernel_height {
                    for kx in 0..kernel_width {
                        let kernel_value = kernel.get_pixel((kx, ky)).unwrap().l;
                        let input_x = x as isize + kx as isize - kernel_half_width as isize;
                        let input_y = y as isize + ky as isize - kernel_half_height as isize;

                        if input_x < 0 || input_y < 0 {
                            continue; // Skip out-of-bounds pixels
                        }

                        let input_x = input_x as usize;
                        let input_y = input_y as usize;

                        if input_x >= input_width || input_y >= input_height {
                            continue; // Skip out-of-bounds pixels
                        }

                        let input_pixel = self.get_pixel((input_x, input_y)).unwrap();
                        if kernel_value == 0.0 {
                            continue;
                        }
                        max_value = max_value.max(input_pixel.l * kernel_value);
                    }
                }

                Luma { l: max_value }
            })
            .collect();

        Image::from_data(input_width, input_height, new_pixels).unwrap()
    }

    fn erode(self, kernel: &Image<Luma>) -> Image<Luma> {
        let (kernel_width, kernel_height) = kernel.dimensions();
        if kernel_width % 2 == 0 || kernel_height % 2 == 0 {
            panic!("Kernel size must be odd in both dimensions");
        }
        let (input_width, input_height) = self.dimensions();
        if input_width < kernel_width || input_height < kernel_height {
            panic!("Input image must be larger than the kernel");
        }

        let kernel_half_width = kernel_width / 2;
        let kernel_half_height = kernel_height / 2;

        let new_pixels = self
            .par_pixels()
            .enumerate()
            .map(|(idx, _pixel)| {
                let mut min_value: f32 = f32::MAX;
                let (x, y) = (idx % input_width, idx / input_width);

                for ky in 0..kernel_height {
                    for kx in 0..kernel_width {
                        let kernel_value = kernel.get_pixel((kx, ky)).unwrap().l;
                        let input_x = x as isize + kx as isize - kernel_half_width as isize;
                        let input_y = y as isize + ky as isize - kernel_half_height as isize;

                        if input_x < 0 || input_y < 0 {
                            continue; // Skip out-of-bounds pixels
                        }

                        let input_x = input_x as usize;
                        let input_y = input_y as usize;

                        if input_x >= input_width || input_y >= input_height {
                            continue; // Skip out-of-bounds pixels
                        }

                        let input_pixel = self.get_pixel((input_x, input_y)).unwrap();
                        if kernel_value == 0.0 {
                            continue;
                        }
                        min_value = min_value.min(input_pixel.l * kernel_value);
                    }
                }

                Luma { l: min_value }
            })
            .collect();

        Image::from_data(input_width, input_height, new_pixels).unwrap()
    }
}
