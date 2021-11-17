use std::fs::OpenOptions;
use std::error::Error;
use memmap::MmapOptions;

pub mod mailbox;
pub mod registers;
pub mod pantilt;

fn main() -> Result<(), Box<dyn Error>> {
    let mut result: Option<u32> = None;
    {
        println!("Beginning of program");
        let mbox = mailbox::open()?;
        println!("After opening mailbox");
        let handle = mailbox::mem_alloc(
            &mbox, 64, 8,
            mailbox::MEM_FLAG_DIRECT | mailbox::MEM_FLAG_HINT_PERMALOCK)?;
        println!("After allocating memory");
        let ptr = mailbox::mem_lock(&mbox, handle)?;
        println!("After locking memory");
        let phys = mailbox::bus_to_phys(ptr);

        {
            let mut program = mailbox::map(phys, 64)?;
            println!("After mapping memory");
            program[0] = 0x00;
            program[1] = 0x08;
            program[2] = 0x5a;
            program[3] = 0x00;
            println!("After writing to memory");
        }
        println!("After unmapping memory");

        result = Some(mailbox::execute_code(&mbox, ptr, phys, 0, 0, 0, 0, 0)?);
        println!("After executing code");

        mailbox::mem_unlock(&mbox, handle)?;
        println!("After unlocking memory");
        mailbox::mem_free(&mbox, handle)?;
        println!("After freeing memory");
    }
    println!("After closing mailbox");

    for _ in 0 .. 1000 {
        println!("Result is {:?}\n", result);
    }

    // pantilt::main()?;

    // let mem = OpenOptions::new().read(true).write(true).open("/dev/mem")?;
    // let mut mmap = unsafe {
    //     MmapOptions::new().offset(0xfe00_0000).len(16 * 1024 * 1024)
    //         .map_mut(&mem)?
    // };
    // let gpio = unsafe {
    //     registers::GPIO::new(mmap.as_mut_ptr() as usize)
    // };
    // println!("FSEL0: {:?}", gpio.get_fsel0());
    // println!("FSEL1: {:?}", gpio.get_fsel1());
    // println!("FSEL2: {:?}", gpio.get_fsel2());
    // println!("FSEL3: {:?}", gpio.get_fsel3());
    // println!("FSEL4: {:?}", gpio.get_fsel4());
    // println!("FSEL5: {:?}", gpio.get_fsel5());
    // println!("IC0 VADDR: {}", gpio.get_ic0_vaddr());
    // println!("IC1 VADDR: {}", gpio.get_ic1_vaddr());
    Ok(())
}
