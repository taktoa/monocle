use std::fs::OpenOptions;
use std::error::Error;
use std::os::unix::io::AsRawFd;
use memmap::MmapOptions;
//use byteorder::{WriteBytesExt, LittleEndian};

// #define MAJOR_NUM 100
// #define IOCTL_MBOX_PROPERTY _IOWR(MAJOR_NUM, 0, char *)
// #define DEVICE_FILE_NAME "/dev/vcio"

nix::ioctl_readwrite!(mbox_property_ioctl, 100, 0, u32);

pub fn mbox_property(file: &std::fs::File, buffer: &mut [u32]) -> nix::Result<i32> {
    println!("mbox_prop: {:#x}", buffer.as_mut_ptr() as usize);

    let mut orig_buffer = Vec::new();
    orig_buffer.resize(buffer.len(), 0);
    orig_buffer.copy_from_slice(buffer);

    unsafe {
        let p = buffer.as_mut_ptr();
        libc::ioctl(file.as_raw_fd(), 0xc0086400, p);
        //mbox_property_ioctl(file.as_raw_fd(), p)
    }
    for i in 0..(buffer[0]/4) {
        println!("p[{}] = {:#x} -> {:#x}", i, orig_buffer[i as usize], buffer[i as usize]);
    }
    Ok(0)
}

fn slice_to_size(slice: &mut [u32], len: u32) -> &mut [u32] {
    slice.split_at_mut(len as usize).0
}

// fn slice_to_size(slice: &mut [u32], len: u32) -> &mut [u8] {
//     unsafe {
//         std::mem::transmute(slice.split_at_mut(len as usize).0)
//     }
// }

pub const MEM_FLAG_DISCARDABLE: u32      = 1 << 0; // can be resized to 0 at any time. Use for cached data
pub const MEM_FLAG_NORMAL: u32           = 0 << 2; // normal allocating alias. Don't use from ARM
pub const MEM_FLAG_DIRECT: u32           = 1 << 2; // 0xC alias uncached
pub const MEM_FLAG_COHERENT: u32         = 2 << 2; // 0x8 alias. Non-allocating in L2 but coherent
pub const MEM_FLAG_L1_NONALLOCATING: u32 = MEM_FLAG_DIRECT | MEM_FLAG_COHERENT; // Allocating in L2
pub const MEM_FLAG_ZERO: u32             = 1 << 4; // initialise buffer to all zeros
pub const MEM_FLAG_NO_INIT: u32          = 1 << 5; // don't initialise (default is initialise to all ones
pub const MEM_FLAG_HINT_PERMALOCK: u32   = 1 << 6; // Likely to be locked for long periods of time.

pub fn mem_alloc(file: &std::fs::File, num_bytes: u32, align: u32, flags: u32) -> nix::Result<u32> {
    let mut p: [u32; 32] = [0; 32];

    let size = 9;
    p[0] = size * 4;   // message size
    p[1] = 0x00000000; // process request
    p[2] = 0x3000C;    // tag ID
    p[3] = 12;         // size of buffer
    p[4] = 12;         // size of data
    p[5] = num_bytes;  // number of bytes to allocate
    p[6] = align;      // alignment
    p[7] = flags;      // MEM_FLAG_L1_NONALLOCATING
    p[8] = 0x00000000; // end tag

    mbox_property(file, slice_to_size(&mut p, size))?;

    Ok(p[5])
}

pub fn mem_free(file: &std::fs::File, handle: u32) -> nix::Result<u32> {
    let mut p: [u32; 32] = [0; 32];

    let size = 7;
    p[0] = size * 4;   // message size
    p[1] = 0x00000000; // process request
    p[2] = 0x3000F;    // tag ID
    p[3] = 4;          // size of buffer
    p[4] = 4;          // size of data
    p[5] = handle;     // handle
    p[6] = 0x00000000; // end tag

    mbox_property(file, slice_to_size(&mut p, size))?;

    Ok(p[5])
}

pub fn mem_lock(file: &std::fs::File, handle: u32) -> nix::Result<u32> {
    let mut p: [u32; 32] = [0; 32];

    let size = 7;
    p[0] = size * 4;   // message size
    p[1] = 0x00000000; // process request
    p[2] = 0x3000D;    // tag ID
    p[3] = 4;          // size of buffer
    p[4] = 4;          // size of data
    p[5] = handle;     // handle
    p[6] = 0x00000000; // end tag

    mbox_property(file, slice_to_size(&mut p, size))?;
    Ok(p[5])
}

pub fn mem_unlock(file: &std::fs::File, handle: u32) -> nix::Result<u32> {
    let mut p: [u32; 32] = [0; 32];

    let size = 7;
    p[0] = size * 4;   // message size
    p[1] = 0x00000000; // process request
    p[2] = 0x3000E;    // tag ID
    p[3] = 4;          // size of buffer
    p[4] = 4;          // size of data
    p[5] = handle;     // handle
    p[6] = 0x00000000; // end tag

    mbox_property(file, slice_to_size(&mut p, size))?;
    Ok(p[5])
}

pub fn execute_code(
    file: &std::fs::File,
    code: u32,
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
) -> nix::Result<u32> {
    let mut p: [u32; 32] = [0; 32];

    let size = 13;
    p[ 0] = size * 4;   // message size
    p[ 1] = 0x00000000; // process request
    p[ 2] = 0x30010;    // tag ID
    p[ 3] = 28;         // size of buffer
    p[ 4] = 28;         // size of data
    p[ 5] = code;       // code
    p[ 6] = r0;         // r0
    p[ 7] = r1;         // r1
    p[ 8] = r2;         // r2
    p[ 9] = r3;         // r3
    p[10] = r4;         // r4
    p[11] = r5;         // r5
    p[12] = 0x00000000; // end tag

    println!("execute {:#x}({:#x},{:#x},{:#x},{:#x},{:#x},{:#x})", code, r0, r1, r2, r3, r4, r5);

    mbox_property(file, slice_to_size(&mut p, size))?;
    Ok(p[5])
}

pub fn qpu_enable(file: &std::fs::File, enable: u32) -> nix::Result<u32> {
    let mut p: [u32; 32] = [0; 32];

    let size = 7;
    p[0] = size * 4;   // message size
    p[1] = 0x00000000; // process request
    p[2] = 0x30012;    // tag ID
    p[3] = 4;          // size of buffer
    p[4] = 4;          // size of data
    p[5] = enable;     // handle
    p[6] = 0x00000000; // end tag

    mbox_property(file, slice_to_size(&mut p, size))?;
    Ok(p[5])
}

pub fn execute_qpu(
    file: &std::fs::File,
    num_qpus: u32,
    control: u32,
    no_flush: u32,
    timeout: u32,
) -> nix::Result<u32> {
    let mut p: [u32; 32] = [0; 32];

    let size = 10;
    p[0] = size * 4;   // message size
    p[1] = 0x00000000; // process request
    p[2] = 0x30011;    // tag ID
    p[3] = 16;         // size of buffer
    p[4] = 16;         // size of data
    p[5] = num_qpus;   // number of QPUs
    p[6] = control;    // FIXME: doc
    p[7] = no_flush;   // FIXME: doc
    p[8] = timeout;    // timeout in milliseconds
    p[9] = 0x00000000; // end tag

    mbox_property(file, slice_to_size(&mut p, size))?;
    Ok(p[5])
}

/*
pub fn poke(file: &std::fs::File, addr: u32, value: u32) -> nix::Result<()> {
    let mut p: [u32; 32] = [0; 32];

    let size = 9;
    p[0] = size * 4;   // message size
    p[1] = 0x00000000; // process request
    p[2] = 0x00038045; // tag ID
    p[3] = 3*4;        // size of buffer
    p[4] = 3*4;        // size of data
    p[5] = addr;       // base-address
    p[6] = 1;          // length maybe?
    p[7] = value;      // value to write
    p[8] = 0x00000000; // end tag

    mbox_property(file, slice_to_size(&mut p, size))?;
    assert!(p[6] != 1);
    Ok(())
}

pub fn peek(file: &std::fs::File, addr: u32) -> nix::Result<u32> {
    let mut p: [u32; 32] = [0; 32];

    let size = 10;
    p[0] = size * 4;   // message size
    p[1] = 0x00000000; // process request
    p[2] = 0x00030045; // tag ID
    p[3] = 4*4;        // size of buffer
    p[4] = 0x0;        // size of data
    p[5] = addr;       // base-address
    p[6] = 2;          // length
    p[7] = 0xdeadbeef; // value to write
    p[8] = 0xdeadbeef; // value to write
    p[9] = 0x00000000; // end tag

    mbox_property(file, slice_to_size(&mut p, size))?;
    assert!(p[6] != 1);
    Ok(p[7])
}
*/

pub fn open() -> Result<std::fs::File, Box<dyn Error>> {
    let vcio = OpenOptions::new().read(true).write(true).open("/dev/vcio")?;
    Ok(vcio)
}

pub fn map(mut base: u32, mut size: u32) -> std::io::Result<memmap::MmapMut> {
    let offset = base % (4 * 1024);
    base -= offset;
    size += offset;
    assert!(offset == 0);

    let mem = OpenOptions::new().read(true).write(true).open("/dev/mem")?;
    unsafe {
        MmapOptions::new().offset(base as u64).len(size as usize)
            .map_mut(&mem)
    }
}

pub fn bus_to_phys(addr: u32) -> u32 {
    addr & (!0xC0000000)
}
