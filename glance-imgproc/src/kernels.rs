use glance_core::img::{Image, pixel::Luma};

pub fn sobel_x() -> Image<Luma> {
    Image::from_data(
        3,
        3,
        [-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0]
            .iter()
            .map(|&l| Luma { l })
            .collect(),
    )
    .unwrap()
}

pub fn sobel_y() -> Image<Luma> {
    Image::from_data(
        3,
        3,
        [-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0]
            .iter()
            .map(|&l| Luma { l })
            .collect(),
    )
    .unwrap()
}

pub fn laplacian_3x3() -> Image<Luma> {
    Image::from_data(
        3,
        3,
        [0.0, 1.0, 0.0, 1.0, -4.0, 1.0, 0.0, 1.0, 0.0]
            .iter()
            .map(|&l| Luma { l })
            .collect(),
    )
    .unwrap()
}

pub fn box_filter(size: usize) -> Image<Luma> {
    let value = 1.0 / (size * size) as f32;
    let data: Vec<Luma> = vec![Luma { l: value }; size * size];

    Image::from_data(size, size, data).unwrap()
}

pub fn gaussian_filter(size: usize, sigma: f32) -> Image<Luma> {
    let mut data = Vec::with_capacity(size * size);
    let half_size = size as f32 / 2.0;
    let two_sigma_squared = 2.0 * sigma * sigma;

    for y in 0..size {
        for x in 0..size {
            let x_diff = (x as f32 - half_size).powi(2);
            let y_diff = (y as f32 - half_size).powi(2);
            let value = (-(x_diff + y_diff) / two_sigma_squared).exp()
                / (std::f32::consts::PI * two_sigma_squared);
            data.push(Luma { l: value });
        }
    }

    Image::from_data(size, size, data).unwrap()
}

pub enum StructuringElementShape {
    Rectangle,
    Disk,
    Cross,
}
/// Creates a square structuring element of given size, filled with ones.
/// Used in morphological operations. Shape has to be specified.
pub fn structuring_element(shape: StructuringElementShape, size: (usize, usize)) -> Image<Luma> {
    let (width, height) = size;

    match shape {
        StructuringElementShape::Rectangle => {
            Image::from_data(width, height, vec![Luma { l: 1.0 }; width * height]).unwrap()
        }
        StructuringElementShape::Disk => {
            let mut data = Vec::with_capacity(width * height);
            let half_width = width as f32 / 2.0;
            let half_height = height as f32 / 2.0;
            for y in 0..height {
                for x in 0..width {
                    let dx = (x as f32 - half_width).powi(2);
                    let dy = (y as f32 - half_height).powi(2);
                    if dx + dy <= (half_width * half_width) {
                        data.push(Luma { l: 1.0 });
                    } else {
                        data.push(Luma { l: 0.0 });
                    }
                }
            }
            Image::from_data(width, height, data).unwrap()
        }
        StructuringElementShape::Cross => {
            let mut data = vec![Luma { l: 0.0 }; width * height];
            for i in 0..width {
                data[i + (height / 2) * width] = Luma { l: 1.0 };
            }
            for i in 0..height {
                data[(width / 2) + i * width] = Luma { l: 1.0 };
            }
            Image::from_data(width, height, data).unwrap()
        }
    }
}
