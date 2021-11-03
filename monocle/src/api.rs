use serde_derive::{Deserialize, Serialize};
use crate::scanline::{Frame, ScanLine};
use crate::gpio::Reading;
use crate::quantity::{Altitude, Azimuth, RightAscension, Declination, PixelDistance};

#[derive(Debug, Deserialize, Serialize)]
pub struct SerializableRecord {
    pub args: String,
    pub level: usize,
    pub target: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MaskSeq {
    ScanningBox,
    // TODO: add more stuff
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TakePictureReq {
    pub masks: Vec<MaskSeq>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Position {
    Unaligned(Altitude, Azimuth),
    Aligned(RightAscension, Declination),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoToReq {
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandReq {
    pub command: String,
    pub arguments: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CalibrateReq {
    Flicker,
    Latency,
    Cutoff(i32, i32, PixelDistance),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    TakePicture(TakePictureReq),
    GoTo(GoToReq),
    Command(CommandReq),
    Calibrate(CalibrateReq),
    Reboot,
    Reset,
    Close,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TakePictureResp {
    pub pulses: Vec<Vec<((Frame, ScanLine), Reading)>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoToResp {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResp {
    pub status: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalibrateResp {
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    TakePicture(TakePictureResp),
    GoTo(GoToResp),
    Command(CommandResp),
    Calibrate(CalibrateResp),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    Log(SerializableRecord),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Receivable {
    Response(Response),
    Event(Event),
}
