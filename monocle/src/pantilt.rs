use std::io::{Read, Write};
use scan_fmt::{scan_fmt, parse::ScanError};
use serial;
use chrono;
use rand;
use crate::quantity::{Azimuth, Altitude, RightAscension, Declination};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u8,
    minor: u8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Device {
    AzimuthMotor,
    AltitudeMotor,
    GPSUnit,
    RealTimeClock,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Model {
    GPSSeries,
    ISeries,
    ISeriesSE,
    CGE,
    AdvancedGT,
    SLT,
    CPC,
    GT,
    SE45,
    SE68,
}

impl Model {
    pub fn from_char(c: char) -> Option<Self> {
        match c as u8 {
            1 => Some(Model::GPSSeries),
            3 => Some(Model::ISeries),
            4 => Some(Model::ISeriesSE),
            5 => Some(Model::CGE),
            6 => Some(Model::AdvancedGT),
            7 => Some(Model::SLT),
            9 => Some(Model::CPC),
            10 => Some(Model::GT),
            11 => Some(Model::SE45),
            12 => Some(Model::SE68),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TrackingMode {
    Off,
    AltAz,
    EqNorth,
    EqSouth,
}

impl TrackingMode {
    pub fn to_char(&self) -> char {
        match *self {
            TrackingMode::Off => 0 as char,
            TrackingMode::AltAz => 1 as char,
            TrackingMode::EqNorth => 2 as char,
            TrackingMode::EqSouth => 3 as char,
        }
    }
}

pub struct Connection {
    pub port: serial::SystemPort,
    pub version: Version,
    pub tracking: TrackingMode,
    pub aligned: bool,
}

#[derive(Debug)]
pub enum Error {
    /// The telescope needs to be aligned.
    NeedsAlignment,
    /// Not only was the response invalid, but it didn't even contain a `#` at
    /// the `n + 2` byte where `n` is the expected message body length.
    GarbageResponse,
    /// We received a response with body length `n + 1` where `n` is the
    /// expected message body length.
    InvalidResponse,
    EchoFailure,
    UTF8(std::string::FromUtf8Error),
    Scan(ScanError),
    Serial(serial::Error),
    IO(std::io::Error),
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Error {
        Error::UTF8(e)
    }
}

impl From<ScanError> for Error {
    fn from(e: ScanError) -> Error {
        Error::Scan(e)
    }
}

impl From<serial::Error> for Error {
    fn from(e: serial::Error) -> Error {
        Error::Serial(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IO(e)
    }
}

impl Connection {
    pub fn new() -> Result<Connection, Error> {
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
        port.set_timeout(std::time::Duration::from_millis(3500))?;
        let mut result = Connection {
            port,
            version: Version { major: 0, minor: 0 },
            tracking: TrackingMode::Off,
            aligned: false,
        };

        // Check echo
        result.echo(rand::random::<u8>())?;

        // Get version
        {
            let response = result.send("V", 2)?;
            let (major, minor) = (
                response.bytes().nth(0).unwrap(),
                response.bytes().nth(1).unwrap(),
            );
            result.version = Version { major, minor };
        }

        // Set the telescope's RTC
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
                                                q, r, s, t, u, v, w, x), 0)?;
            assert!(response.is_empty());
        }

        // Get whether we're in tracking mode
        {
            assert!((result.version.major > 2)
                    || ((result.version.major == 2) &&
                        (result.version.minor >= 3)));
            let response = result.send("t", 1)?;
            match response.bytes().nth(0).unwrap() {
                0 => { result.tracking = TrackingMode::Off; },
                1 => { result.tracking = TrackingMode::AltAz; },
                2 => { result.tracking = TrackingMode::EqNorth; },
                3 => { result.tracking = TrackingMode::EqSouth; },
                _ => assert!(false),
            }

       }

        // Get whether we're aligned
        {
            assert!((result.version.major > 1)
                    || ((result.version.major == 1) &&
                        (result.version.minor >= 2)));
            let response = result.send("J", 1)?;
            match response.bytes().nth(0).unwrap() {
                0 => { result.aligned = false; },
                1 => { result.aligned = true; },
                _ => assert!(false),
            }
        }

        Ok(result)
    }

    pub fn send(
        &mut self, message: &str, response_size: usize
    ) -> Result<String, Error> {
        use std::io::{Read, Write};
        self.port.write_all(message.as_bytes())?;
        self.port.flush()?;
        let mut buf = Vec::<u8>::new();
        buf.resize(response_size + 1, 0);
        self.port.read_exact(&mut buf)?;
        if *(buf.last().unwrap()) != b'#' {
            let mut extra: [u8; 1] = [0; 1];
            self.port.read_exact(&mut extra)?;
            println!("DEBUG: {}", String::from_utf8(buf.split_at(response_size).0.to_vec())?);
            if extra[0] != b'#' {
                return Err(Error::InvalidResponse);
            } else {
                return Err(Error::GarbageResponse);
            }
        }
        Ok(String::from_utf8(buf.split_at(response_size).0.to_vec())?)
    }

    pub fn echo(&mut self, byte: u8) -> Result<(), Error> {
        let result = self.send(&format!("K{}", byte as char), 1)?;
        if result != format!("{}", byte as char) {
            return Err(Error::EchoFailure);
        }
        Ok(())
    }

    pub fn version(&mut self) -> Version {
        self.version.clone()
    }

    pub fn get_ra_dec(&mut self) -> Result<(RightAscension, Declination), Error> {
        assert!(self.version >= (Version { major: 1, minor: 2 }));
        if self.version >= (Version { major: 1, minor: 6 }) {
            let precise = self.send("e", 0)?;
            Ok(precise_nexstar_to_dd(&precise)?)
        } else {
            let imprecise = self.send("E", 0)?;
            Ok(imprecise_nexstar_to_dd(&imprecise)?)
        }
    }

    pub fn get_az_alt(&mut self) -> Result<(Azimuth, Altitude), Error> {
        assert!(self.version >= (Version { major: 1, minor: 2 }));
        if self.version >= (Version { major: 2, minor: 2 }) {
            let precise = self.send("z", 17)?;
            Ok(precise_nexstar_to_dd(&precise)?)
        } else {
            let imprecise = self.send("Z", 9)?;
            Ok(imprecise_nexstar_to_dd(&imprecise)?)
        }
    }

    pub fn goto_ra_dec(
        &mut self, ra: RightAscension, dec: Declination
    ) -> Result<(), Error> {
        assert!(self.aligned);
        assert!(self.version >= (Version { major: 1, minor: 2 }));
        if self.version >= (Version { major: 1, minor: 6 }) {
            self.send(&format!("r{}", dd_to_precise_nexstar((ra, dec))), 0)?;
        } else {
            self.send(&format!("R{}", dd_to_imprecise_nexstar((ra, dec))), 0)?;
        }
        Ok(())
    }

    pub fn goto_az_alt(
        &mut self, az: Azimuth, alt: Altitude
    ) -> Result<(), Error> {
        assert!(self.version >= (Version { major: 1, minor: 2 }));
        if self.version >= (Version { major: 2, minor: 2 }) {
            self.send(&format!("b{}", dd_to_precise_nexstar((az, alt))), 0)?;
        } else {
            self.send(&format!("B{}", dd_to_imprecise_nexstar((az, alt))), 0)?;
        }
        Ok(())
    }

    pub fn sync(
        &mut self, ra: RightAscension, dec: Declination
    ) -> Result<(), Error> {
        assert!(self.version >= (Version { major: 4, minor: 10 }));
        self.send(&format!("s{}", dd_to_precise_nexstar((ra, dec))), 0)?;
        Ok(())
    }

    pub fn set_tracking(
        &mut self, tracking_mode: TrackingMode
    ) -> Result<(), Error> {
        assert!(self.version >= (Version { major: 1, minor: 6 }));
        self.send(&format!("T{}", tracking_mode.to_char()), 0)?;
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

// pub fn main() -> Result<(), Box<dyn Error>> {
//     let mut conn = Connection::new()?;
//     conn.port.write("B12AB,4000".as_bytes())?;
//     // let mut buf: Vec<u8> = (0 .. 255).map(|_| 0).collect();
//     // port.read(&mut buf);
//     // println!("DEBUG: {:?}", std::str::from_utf8(&buf));
//     Ok(())
// }

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
