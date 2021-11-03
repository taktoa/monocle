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

pub type ScanLine = u32;
pub type Frame = u32;

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

    pub fn read_dispstat(&self, scaler: Scaler) -> u32 {
        let transmuted = self.mmap.as_ptr() as *const u32;
        let offset = match scaler {
            Scaler::Scaler0 => 0x48,
            Scaler::Scaler1 => 0x58,
            Scaler::Scaler2 => 0x68,
        };
        unsafe { transmuted.wrapping_offset(offset / 4).read_volatile() }
    }

    pub fn read_dispctrl(&self, scaler: Scaler) -> u32 {
        let transmuted = self.mmap.as_ptr() as *const u32;
        let offset = match scaler {
            Scaler::Scaler0 => 0x40,
            Scaler::Scaler1 => 0x50,
            Scaler::Scaler2 => 0x60,
        };
        unsafe { transmuted.wrapping_offset(offset / 4).read_volatile() }
    }

    pub fn read_scanline(&self, scaler: Scaler) -> ScanLine {
        let read = self.read_dispstat(scaler);
        read & 0b11_1111_1111_1111
    }
}
