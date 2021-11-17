use std::error::Error;
use std::io::{Read, Write};
use scan_fmt::{scan_fmt, parse::ScanError};
use serial;
use chrono;

type RightAscension = f64;
type Declination = f64;
type Azimuth = f64;
type Altitude = f64;

pub struct Connection {
    port: serial::SystemPort,
    // version: Version,
    // tracking_mode: bool,
}

impl Connection {
    pub fn new() -> serial::Result<Connection> {
        use serial::core::SerialPort;
        let mut port = serial::open("/dev/ttyUSB0")?;
        let settings = serial::PortSettings {
            baud_rate:    serial::BaudRate::Baud9600,
            char_size:    serial::CharSize::Bits8,
            parity:       serial::Parity::ParityNone,
            stop_bits:    serial::StopBits::Stop1,
            flow_control: serial::FlowControl::FlowNone,
        };
        port.configure(&settings)?;
        let mut result = Connection { port };

        {
            use chrono::{Timelike, Datelike};
            let local: chrono::DateTime<chrono::Local> = chrono::Local::now();
            // let tz: i32 = local.timezone().fix().local_minus_utc() / 3600;
            let tz: i32 = -8;
            let q = (local.hour() as u8) as char;
            let r = (local.minute() as u8) as char;
            let s = (local.second() as u8) as char;
            let t = (local.month() as u8) as char;
            let u = (local.day() as u8) as char;
            let v = ((local.year() % 2000) as u8) as char;
            let w = (if tz < 0 { 256 + tz } else { tz }) as u8;
            let x = 0;
            let response = result.send(&format!("H{}{}{}{}{}{}{}{}",
                                                q, r, s, t, u, v, w, x)).unwrap();
            // FIXME: no unwrap ^^^
            assert!(response.is_empty());
        }

        Ok(result)
    }

    pub fn send(&mut self, message: &str) -> Result<String, ()> {
        Ok("".to_string())
    }

    pub fn goto_ra_dec(&mut self, ra: RightAscension, dec: Declination) -> Result<(), ()> {
        // Check that we are aligned
        // Use precise if version supports it, imprecise otherwise
        Ok(())
    }

    pub fn goto_az_alt(&mut self, ra: RightAscension, dec: Declination) -> Result<(), ()> {
        // Use precise if version supports it, imprecise otherwise
        Ok(())
    }

}

fn dd_to_precise_nexstar((mut x, mut y): (f64, f64)) -> String {
    x -= 360.0 * f64::floor(x / 360.0);
    y -= 360.0 * f64::floor(y / 360.0);
    if y < 0.0 {
        y += 360.0;
    }
    let x_encoded = ((x / 360.0) * (0xFFFFFFFFu32 as f64)) as u32;
    let y_encoded = ((y / 360.0) * (0xFFFFFFFFu32 as f64)) as u32;
    format!("{:08X},{:08X}", x_encoded, y_encoded)
}

fn dd_to_imprecise_nexstar((mut x, mut y): (f64, f64)) -> String {
    x -= 360.0 * f64::floor(x / 360.0);
    y -= 360.0 * f64::floor(y / 360.0);
    if y < 0.0 {
        y += 360.0;
    }
    let x_encoded = ((x / 360.0) * 65536.0) as u16;
    let y_encoded = ((y / 360.0) * 65536.0) as u16;
    format!("{:04X},{:04X}", x_encoded, y_encoded)
}

fn precise_nexstar_to_dd(string: &str) -> Result<(f64, f64), ScanError> {
    let (x, y) = scan_fmt!(string, "{x},{x}", [hex u32], [hex u32])?;
    let x_factor = (x as f64) / (0xFFFFFFFFu32 as f64);
    let y_factor = (y as f64) / (0xFFFFFFFFu32 as f64);
    let x_degrees = 360.0 * x_factor;
    let mut y_degrees = 360.0 * y_factor;
    if y_degrees < -90.0001 {
        y_degrees += 360.0;
    }
    if y_degrees > 90.0001 {
        y_degrees -= 360.0;
    }
    Ok((x_degrees, y_degrees))
}

fn imprecise_nexstar_to_dd(string: &str) -> Result<(f64, f64), ScanError> {
    let (x, y) = scan_fmt!(string, "{x},{x}", [hex u32], [hex u32])?;
    let x_factor = (x as f64) / 65536.0;
    let y_factor = (y as f64) / 65536.0;
    let x_degrees = 360.0 * x_factor;
    let mut y_degrees = 360.0 * y_factor;
    if y_degrees < -90.0001 {
        y_degrees += 360.0;
    }
    if y_degrees > 90.0001 {
        y_degrees -= 360.0;
    }
    Ok((x_degrees, y_degrees))
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut conn = Connection::new()?;
    conn.port.write("B12AB,4000".as_bytes())?;
    // let mut buf: Vec<u8> = (0 .. 255).map(|_| 0).collect();
    // port.read(&mut buf);
    // println!("DEBUG: {:?}", std::str::from_utf8(&buf));
    Ok(())
}

#[cfg(test)]
mod tests {
    use approx::abs_diff_eq;

    #[test]
    fn nexstar_dd_roundtrip() {
        for i in 10 .. 80 {
            for j in 10 .. 80 {
                let dd = (i as f64, j as f64);
                let pnex = super::dd_to_precise_nexstar(dd);
                let nex = super::dd_to_imprecise_nexstar(dd);
                println!("pnex: {}, nex: {}", pnex, nex);
                let dd_precise_roundtrip =
                    super::precise_nexstar_to_dd(&pnex).unwrap();
                let dd_imprecise_roundtrip =
                    super::imprecise_nexstar_to_dd(&nex).unwrap();

                abs_diff_eq!(dd.0, dd_precise_roundtrip.0, epsilon = 0.0001);
                abs_diff_eq!(dd.1, dd_precise_roundtrip.1, epsilon = 0.0001);
                abs_diff_eq!(dd.0, dd_imprecise_roundtrip.0, epsilon = 0.01);
                abs_diff_eq!(dd.1, dd_imprecise_roundtrip.1, epsilon = 0.01);
            }
        }
    }
}
