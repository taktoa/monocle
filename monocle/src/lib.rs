#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::OpenOptions;
use std::error::Error;
use std::collections::BTreeSet;
use memmap::MmapOptions;

pub mod mailbox;
pub mod gpio;
pub mod scanline;
pub mod lasso;
//pub mod registers;
pub mod pantilt;
pub mod capture;
pub mod calibrate;
pub mod server;
pub mod client;
pub mod api;
pub mod lcd;
pub mod rotation;
pub mod mask;
pub mod quantity;
pub mod adaptive;

//pub fn print_peek(file: &std::fs::File, addr: u32) {
//    println!("peek({:#x}) == {:#x}", addr, mailbox::peek(file, addr).unwrap());
//}

fn main() -> Result<(), Box<dyn Error>> {
    println!("DEBUG");
    //lasso::main()?;

    // let gpio = gpio::GPIO::new()?;
    // for _ in 0 .. 4000 {
    //     if let Some(photons) = gpio.record_window() {
    //         println!("pulses counted in 100 µs: {:>15}", difference);
    //     }
    // }
    Ok(())
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let mut argument_set = BTreeSet::new();
//     for argument in std::env::args() {
//         argument_set.insert(argument);
//     }
//
//     let mut result: Option<u32> = None;
//
//     {
//         println!("Beginning of program");
//         let mbox = mailbox::open()?;
//         println!("After opening mailbox");
//         let mut flags = mailbox::MEM_FLAG_DIRECT;
//         if argument_set.contains("permalock") {
//             println!("MEM_FLAG_HINT_PERMALOCK is enabled");
//             flags = flags | mailbox::MEM_FLAG_HINT_PERMALOCK;
//         }
//         let handle = mailbox::mem_alloc(
//             &mbox, 64, 4096, flags)?;
//         println!("REMY DEBUG: handle is {}", handle);
//         println!("After allocating memory");
//         let ptr = mailbox::mem_lock(&mbox, handle)?;
//         println!("REMY DEBUG: ptr is {:#x}", ptr);
//         println!("After locking memory");
//
//         if argument_set.contains("mmap_poke") {
//             let mut program = mailbox::map(mailbox::bus_to_phys(ptr), 64)?;
//             println!("After mapping memory");
//             program[0] = 0x00;
//             program[1] = 0x08;
//             program[2] = 0x5a;
//             program[3] = 0x00;
//             println!("After writing to memory");
//         }
//         println!("After unmapping memory");
//
//         let phys = 0x7e200004;
//         println!("REMY DEBUG: phys is {:#x}", phys);
//         if argument_set.contains("execute") {
//             result = Some(mailbox::execute_code(&mbox, ptr, phys, 0, 0, 0, 0, 0)?);
//             println!("After executing code");
//         }
//
//         mailbox::mem_unlock(&mbox, handle)?;
//         println!("After unlocking memory");
//         mailbox::mem_free(&mbox, handle)?;
//         println!("After freeing memory");
//     }
//     println!("After closing mailbox");
//
//     for _ in 0 .. 100 {
//         //println!("Result is {:?}\n", result);
//     }
//
//     // pantilt::main()?;
//
//     // let mem = OpenOptions::new().read(true).write(true).open("/dev/mem")?;
//     // let mut mmap = unsafe {
//     //     MmapOptions::new().offset(0xfe00_0000).len(16 * 1024 * 1024)
//     //         .map_mut(&mem)?
//     // };
//     // let gpio = unsafe {
//     //     registers::GPIO::new(mmap.as_mut_ptr() as usize)
//     // };
//     // println!("FSEL0: {:?}", gpio.get_fsel0());
//     // println!("FSEL1: {:?}", gpio.get_fsel1());
//     // println!("FSEL2: {:?}", gpio.get_fsel2());
//     // println!("FSEL3: {:?}", gpio.get_fsel3());
//     // println!("FSEL4: {:?}", gpio.get_fsel4());
//     // println!("FSEL5: {:?}", gpio.get_fsel5());
//     // println!("IC0 VADDR: {}", gpio.get_ic0_vaddr());
//     // println!("IC1 VADDR: {}", gpio.get_ic1_vaddr());
//     Ok(())
// }
