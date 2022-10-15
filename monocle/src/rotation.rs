use crate::quantity::{Latitude, Azimuth, Altitude, RotationAngle};
use crate::mask::Mask;

pub fn field_rotation_speed(
    lat: Latitude,
    az: Azimuth,
    alt: Altitude,
) -> f64 {
    let earth_rotation_rate = 7.292115826090781e-5;
    let k = earth_rotation_rate * f64::cos(lat);
    k * f64::cos(az) / f64::cos(alt)
}

pub struct FieldRotation {
    start_time: time::Instant,
    angle: RotationAngle,
}

impl FieldRotation {
    pub fn new() -> Self {
        FieldRotation {
            start_time: time::Instant::now(),
            angle: 0.0,
        }
    }

    pub fn get_angle(&self) -> RotationAngle {
        self.angle
    }

    pub fn update_angle(
        &mut self,
        lat: Latitude,
        az: Azimuth,
        alt: Altitude,
    ) -> RotationAngle {
        let now = time::Instant::now();
        self.angle +=
            field_rotation_speed(lat, az, alt)
            * ((now - self.start_time).whole_microseconds() as f64 / 1000000.0);
        self.start_time = now;
        return self.angle;
    }
}
