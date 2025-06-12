use glance_core::img::{
    Image,
    pixel::{Luma, Rgba},
};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};

pub trait LinearFilterExtRgba {
    fn convolve_2d(self, kernel: Image<Luma>) -> Image<Rgba>;
}

impl LinearFilterExtRgba for Image<Rgba> {
    fn convolve_2d(self, kernel: Image<Luma>) -> Image<Rgba> {
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

        let convolved_pixels = self
            .par_pixels()
            .enumerate()
            .map(|(idx, _pixel)| {
                let (x, y) = (idx % input_width, idx / input_width);
                let mut r_sum = 0.0;
                let mut g_sum = 0.0;
                let mut b_sum = 0.0;
                let alpha = _pixel.a;

                for ky in 0..kernel_height {
                    for kx in 0..kernel_width {
                        let kernel_value = kernel.get_pixel((kx, ky)).unwrap();
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

                        if input_x < input_width && input_y < input_height {
                            r_sum += input_pixel.r * kernel_value.l;
                            g_sum += input_pixel.g * kernel_value.l;
                            b_sum += input_pixel.b * kernel_value.l;
                        }
                    }
                }

                Rgba {
                    r: r_sum,
                    g: g_sum,
                    b: b_sum,
                    a: alpha,
                }
            })
            .collect();

        Image::from_data(input_width, input_height, convolved_pixels).unwrap()
    }
}

pub trait ConvolutionExtLuma {
    fn convolve_2d(self, kernel: Image<Luma>) -> Image<Luma>;
}

impl ConvolutionExtLuma for Image<Luma> {
    fn convolve_2d(self, kernel: Image<Luma>) -> Image<Luma> {
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

        let convolved_pixels = self
            .par_pixels()
            .enumerate()
            .map(|(idx, _pixel)| {
                let (x, y) = (idx % input_width, idx / input_width);
                let mut l_sum = 0.0;

                for ky in 0..kernel_height {
                    for kx in 0..kernel_width {
                        let kernel_value = kernel.get_pixel((kx, ky)).unwrap();
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

                        if input_x < input_width && input_y < input_height {
                            l_sum += input_pixel.l * kernel_value.l;
                        }
                    }
                }

                Luma { l: l_sum }
            })
            .collect();

        Image::from_data(input_width, input_height, convolved_pixels).unwrap()
    }
}
