use std::fs::OpenOptions;
use std::error::Error;
use std::collections::BTreeSet;
use memmap::MmapOptions;

// #define SCALER_DISPSTAT0 0x7e400048
// #define SCALER_DISPSTAT1 0x7e400058
// #define SCALER_DISPSTAT2 0x7e400068
// map /dev/mem with an offset of 0xfe400000
// then do a 32 bit read at offset 0x48, 0x58, and 0x68
// one of them will be incrementing
// lowest 12 bits are the scanline #
// next 6 bits are the frame #

#[derive(Clone, Copy)]
pub enum Scaler {
    Scaler0,
    Scaler1,
    Scaler2,
}

#[derive(Clone, Debug)]
pub struct ScanLine {
    frame: u32,
    scanline: u32,
}

impl ScanLine {
    pub fn to_u32(&self) -> u32 {
        (self.frame << 12) + self.scanline
    }
}

pub struct ScanLineMem {
    file: std::fs::File,
    mmap: memmap::Mmap,
}

impl ScanLineMem {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mem = OpenOptions::new().read(true).open("/dev/mem")?;
        let mem_mmap = unsafe {
            MmapOptions::new().offset(0xFE400000).len(0x100).map(&mem)?
        };
        Ok(ScanLineMem { file: mem, mmap: mem_mmap })
    }

    pub fn read_scanline(&self, scaler: Scaler) -> ScanLine {
        unsafe {
            let transmuted = self.mmap.as_ptr() as *const u32;
            let offset = match scaler {
                Scaler::Scaler0 => 0x48,
                Scaler::Scaler1 => 0x58,
                Scaler::Scaler2 => 0x68,
            };
            let read = *(transmuted.offset(offset / 4));
            let scanline = read & 0b1111_1111_1111;
            let frame = (read >> 12) & 0b11_1111;
            ScanLine { scanline, frame }
        }
    }
}
