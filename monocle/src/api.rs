use serde_derive::{Deserialize, Serialize};
use crate::quantity::{Altitude, Azimuth, RightAscension, Declination};

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
pub enum Request {
    TakePicture(TakePictureReq),
    GoTo(GoToReq),
    Reboot,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TakePictureResp {
    pub pulses: Vec<Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoToResp {
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    TakePicture(TakePictureResp),
    GoTo(GoToResp),
}
