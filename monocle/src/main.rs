use std::fs::OpenOptions;
use std::io::Write;
use std::error::Error;
use memmap::MmapOptions;

pub mod registers;

fn main() -> Result<(), Box<dyn Error>> {
    let mem = OpenOptions::new().read(true).write(true).open("/dev/mem")?;
    let mut mmap = unsafe {
        MmapOptions::new().offset(0xfe00_0000).len(16 * 1024 * 1024)
            .map_mut(&mem)?
    };
    let gpio = unsafe {
        registers::GPIO::new(mmap.as_mut_ptr() as usize)
    };
    println!("IC0 VADDR: {}", gpio.get_ic0_vaddr());
    println!("IC1 VADDR: {}", gpio.get_ic1_vaddr());
    Ok(())
}
