use std::fs::OpenOptions;
use std::error::Error;
use std::collections::BTreeSet;
use memmap::MmapOptions;

pub struct GPIO {
    file: std::fs::File,
    mmap: memmap::Mmap,
}

impl GPIO {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let gpio = OpenOptions::new().read(true).open("/dev/gpiomem")?;
        let gpio_mmap = unsafe {
            MmapOptions::new().offset(0).len(1024).map(&gpio)?
        };
        Ok(GPIO { file: gpio, mmap: gpio_mmap })
    }

    pub fn read_gpio(&self) -> u32 {
        unsafe {
            let transmuted =
                std::mem::transmute::<*const u8, *const u32>(
                    self.mmap.as_ptr());
            *(transmuted.offset(13))
        }
    }

    // Records the number of pulses in a 100 microsecond time window
    pub fn record_window(&self) -> Option<u32> {
        let before_state = self.read_gpio();
        std::thread::sleep(std::time::Duration::from_micros(100));
        let after_state = self.read_gpio();
        let before = compute_counter(before_state);
        let after = compute_counter(after_state);
        let difference = if after < before {
            before + (4096 - after)
        } else {
            after - before
        };
        if difference < 4096 {
            return Some(difference);
        }
        None
    }
}

pub fn compute_counter(gpio: u32) -> u32 {
    let q1: u32 = (gpio >> 17) & 1;
    let q2: u32 = (gpio >> 5) & 1;
    let q3: u32 = (gpio >> 6) & 1;
    let q4: u32 = (gpio >> 13) & 1;
    let q5: u32 = (gpio >> 26) & 1;
    let q6: u32 = (gpio >> 12) & 1;
    let q7: u32 = (gpio >> 19) & 1;
    let q8: u32 = (gpio >> 22) & 1;
    let q9: u32 = (gpio >> 18) & 1;
    let q10: u32 = (gpio >> 23) & 1;
    let q11: u32 = (gpio >> 24) & 1;
    let q12: u32 = (gpio >> 16) & 1;
    q1 + (q2 << 1) + (q3 << 2) + (q4 << 3) + (q5 << 4)
        + (q6 << 5) + (q7 << 6) + (q8 << 7) + (q9 << 8) + (q10 << 9)
        + (q11 << 10) + (q12 << 11)
}
