use glance_core::img::{Image, pixel::Luma};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};

pub trait AffineTransformationsExtLuma {
    /// Applies an affine transformation to the image.
    fn affine_transform(self, matrix: [[f32; 3]; 3]) -> Image<Luma>;
    fn rotate(self, angle: f32) -> Image<Luma>;
    fn scale(self, scale: (f32, f32)) -> Image<Luma>;
    fn translate(self, offset: (f32, f32)) -> Image<Luma>;
}

impl AffineTransformationsExtLuma for Image<Luma> {
    fn affine_transform(self, matrix: [[f32; 3]; 3]) -> Image<Luma> {
        let (width, height) = self.dimensions();
        let new_data = self
            .par_pixels()
            .enumerate()
            .map(|(idx, _pixel)| {
                let (x, y) = (idx % width, idx / width);
                let new_x =
                    (matrix[0][0] * x as f32 + matrix[0][1] * y as f32 + matrix[0][2]) as usize;
                let new_y =
                    (matrix[1][0] * x as f32 + matrix[1][1] * y as f32 + matrix[1][2]) as usize;

                if new_x < width && new_y < height && new_x > 0 && new_y > 0 {
                    return self.get_pixel((new_x, new_y)).unwrap().clone();
                }

                Luma { l: 0.0 }
            })
            .collect::<Vec<Luma>>();

        Image::from_data(width, height, new_data).unwrap()
    }

    fn rotate(self, angle: f32) -> Image<Luma> {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let matrix = [
            [cos_angle, -sin_angle, 0.0],
            [sin_angle, cos_angle, 0.0],
            [0.0, 0.0, 1.0],
        ];

        self.affine_transform(matrix)
    }

    fn scale(self, scale: (f32, f32)) -> Image<Luma> {
        let matrix = [[scale.0, 0.0, 0.0], [0.0, scale.1, 0.0], [0.0, 0.0, 1.0]];

        self.affine_transform(matrix)
    }

    fn translate(self, offset: (f32, f32)) -> Image<Luma> {
        let matrix = [[1.0, 0.0, offset.0], [0.0, 1.0, offset.1], [0.0, 0.0, 1.0]];

        self.affine_transform(matrix)
    }
}
