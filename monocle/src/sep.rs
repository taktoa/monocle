use crate::quantity::{PixelDistance, RotationAngle};
use rand::Rng;
use image::Luma;
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};

pub type Mask = imageproc::definitions::Image<image::Luma<u8>>;

pub mod aperture {
    pub struct EllipseAxes {
        pub a: f64,
        pub b: f64,
        pub theta: f64,
    }

    pub fn ellipse_axes(cxx: f64, cyy: f64, cxy: f64) -> Option<EllipseAxes> {
        let p = cxx + cyy;
        let q = cxx - cyy;
        let t = f64::sqrt(q * q + cxy * cxy);
        if ((cxx * cyy - ((cxy * cxy) / 4.0)) <= 0) || (p <= 0) {
            return None;
        }
        let a = f64::sqrt(2.0 / (p - t));
        let b = f64::sqrt(2.0 / (p + t));
        let mut theta: f64 = if cxy == 0.0 || q == 0.0 {
            0.0
        } else {
            f64::atan(cxy / q) / 2.0
        };
        if (cxx > cyy) {
            theta += std::f64::consts::FRAC_PI_2;
        }
        if (theta > std::f64::consts::FRAC_PI_2) {
            theta -= std::f64::consts::PI;
        }
        return EllipseAxes { a, b, theta };
    }
}
