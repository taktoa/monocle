use std::fs::OpenOptions;
use std::error::Error;
use std::collections::BTreeSet;
use memmap::MmapOptions;
use serde_derive::{Deserialize, Serialize};

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
        let transmuted = self.mmap.as_ptr() as *const u32;
        unsafe { transmuted.offset(13).read_volatile() }
    }

    // Records the number of pulses in a 100 microsecond time window
    pub fn record_window(&self) -> Option<Reading> {
        let before_time = std::time::Instant::now();
        let before_state = self.read_gpio();
        std::thread::sleep(std::time::Duration::from_micros(100));
        let after_time = std::time::Instant::now();
        let after_state = self.read_gpio();
        let duration = after_time.duration_since(before_time);
        let before = compute_counter(before_state);
        let after = compute_counter(after_state);
        let difference = if after.counter < before.counter {
            before.counter + (4096 - after.counter)
        } else {
            after.counter - before.counter
        };
        let ratio = duration.as_micros() as f64 / 100.0;
        if ratio > 2.5 {
            return None;
        }
        Some(Reading {
            overlight: before.overlight || after.overlight,
            counter: (difference as f64 * ratio) as u32,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Reading {
    pub overlight: bool,
    pub counter: u32,
}

pub fn compute_counter(gpio: u32) -> Reading {
    let overlight: bool = ((gpio >> 2) & 1) == 1;
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
    let counter = q1 + (q2 << 1) + (q3 << 2) + (q4 << 3) + (q5 << 4)
        + (q6 << 5) + (q7 << 6) + (q8 << 7) + (q9 << 8) + (q10 << 9)
        + (q11 << 10) + (q12 << 11);
    Reading { overlight, counter }
}
