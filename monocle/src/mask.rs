use crate::quantity::{PixelDistance, RotationAngle};
use rand::Rng;
use image::Luma;
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};

pub type Mask = imageproc::definitions::Image<image::Luma<u8>>;

pub fn binary_random_mask<T: Rng>(
    width: usize,
    height: usize,
) -> Mask {
    let mut result = Mask::new(width as u32, height as u32);
    for pixel in result.pixels_mut() {
        *pixel = Luma([if rand::random() { 255u8 } else { 0u8 }]);
    }
    result
}

pub fn grayscale_uniform_random_mask<T: Rng>(
    width: usize,
    height: usize,
) -> Mask {
    let mut result = Mask::new(width as u32, height as u32);
    for pixel in result.pixels_mut() {
        *pixel = Luma([rand::random::<u8>()]);
    }
    result
}

pub fn rotate_mask(mask: &Mask, angle: RotationAngle) -> Mask {
    rotate_about_center(mask, angle as f32, Interpolation::Bicubic, Luma([0]))
}

// pub fn apply_square_cutoff(mask: &mut Mask, center: (i32, i32), cutoff: PixelDistance) {
//     fn inf_norm_distance(a: (f64, f64), b: (f64, f64)) -> f64 {
//         let delta_x = a.0 - b.0;
//         let delta_y = a.1 - b.1;
//         std::cmp::max(delta_x, delta_y)
//     }
//
//     for (x, y, pixel) in mask.enumerate_pixels_mut() {
//         let position = (x as f64, y as f64);
//         if inf_norm_distance(position, (center.0 as f64, center.1 as f64)) > cutoff {
//             *pixel = Luma([0]);
//         }
//     }
// }

pub fn apply_circular_cutoff(mask: &mut Mask, cutoff: PixelDistance) {
    fn distance_squared(a: (f64, f64), b: (f64, f64)) -> f64 {
        let delta_x = a.0 - b.0;
        let delta_y = a.1 - b.1;
        delta_x * delta_x + delta_y * delta_y
    }

    let (w, h) = mask.dimensions();
    let center = (w as f64 / 2.0, h as f64 / 2.0);
    let cutoff_squared = cutoff * cutoff;
    for (x, y, pixel) in mask.enumerate_pixels_mut() {
        let position = (x as f64, y as f64);
        if distance_squared(position, center) > cutoff_squared {
            *pixel = Luma([0]);
        }
    }
}
