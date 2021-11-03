use crate::rotation::FieldRotation;
use crate::scanline::{Frame, ScanLine, ScanLineMem, Scaler};
use crate::gpio::{GPIO, Reading};
use std::sync::{Arc, Barrier, atomic::{AtomicBool, AtomicU32, Ordering}};
use log::*;
// center = (0, 210), radius = 750

pub const DIVIDER: u32 = 6;

pub fn scanbox() -> Vec<((Frame, ScanLine), Reading)> {
    let barrier = Arc::new(Barrier::new(2));
    let barrier_copy = barrier.clone();

    let frame_counter = Arc::new(AtomicU32::new(0));
    let frame_counter_copy = frame_counter.clone();

    let kill_channel = Arc::new(AtomicBool::new(false));
    let kill_channel_copy = kill_channel.clone();

    let handle = std::thread::spawn(move || {
        barrier.wait();
        let slm = ScanLineMem::new().unwrap();
        let gpio = GPIO::new().unwrap();
        let mut photon_counts: Vec<((Frame, ScanLine), Reading)> = Vec::new();
        // let start_frame = frame_counter_copy.load(Ordering::SeqCst);
        photon_counts.reserve(50_000_000);
        while !kill_channel_copy.load(Ordering::SeqCst) {
            let scanline = slm.read_scanline(Scaler::Scaler0);
            let frame = frame_counter_copy.load(Ordering::SeqCst);
            if let Some(pulses) = gpio.record_window() {
                photon_counts.push(((frame, scanline), pulses));
            }
            // trace!("Recorded photon window in scanline: {:?}", scanline);
        }
        info!("Finished photon counting");
        photon_counts
    });

    let mut counter = 0;
    crate::lcd::run(|dm| {
        if counter % 50 == 0 {
            info!("Reached frame {}", counter);
        }
        if counter == 0 {
            barrier_copy.wait();
        }
        assert_eq!(1500 % DIVIDER, 0);
        let x = counter % (1500 / DIVIDER);
        let y = counter / (1500 / DIVIDER);
        if (x >= (1500 / DIVIDER)) || (y >= (1500 / DIVIDER)) {
            return true;
        }
        let set_line = |buf: &mut [u8], start: u32, width: u32, value: u8| {
            // if start > buf.len() as u32 {
            //     panic!("DEBUG: start ({}) > buf.len() ({})", start, buf.len());
            // }
            // let foo = buf.split_at_mut(start as usize).1;
            // if width > foo.len() as u32 {
            //     panic!("DEBUG: width ({}) > foo.len() ({})", width, foo.len());
            // }
            buf
                .split_at_mut(start as usize).1
                .split_at_mut(width as usize).0
                .fill(value);
        };
        let set_block =
            |buf: &mut [u8], x: u32, y: u32, width: u32, height: u32, value: u8| {
                let upper_left = x + y * 4320;
                for row in 0 .. height {
                    set_line(buf, upper_left + row * 4320, width, value);
                }
            };
        // if !((x == 0) && (y == 0)) {
        //     let (previous_x, previous_y) = if x == 0 {
        //         ((4320 / DIVIDER) - 1, y - 1)
        //     } else {
        //         (x - 1, y)
        //     };
        //     set_block(dm.as_mut(), previous_x, previous_y, 0);
        // }
        dm.as_mut().fill(0);
        set_block(dm.as_mut(),
                  x * DIVIDER + 1410,
                  y * DIVIDER + 740,
                  DIVIDER,
                  DIVIDER,
                  255);
        // for (i, pixel) in dm.as_mut().iter_mut().enumerate() {
        //     let pixel_x = i % 4320;
        //     let pixel_y = i / 4320;
        //     *pixel = if (pixel_x / DIVIDER == x) && (pixel_y / DIVIDER == y) {
        //         255u8
        //     } else {
        //         0u8
        //     };
        // }
        counter += 1;
        frame_counter.fetch_add(1, Ordering::SeqCst);
        false
        // counter >= (11_059_200 / (DIVIDER * DIVIDER))
    }).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(100));

    kill_channel.store(true, Ordering::SeqCst);

    info!("Finished LCD display stuff");

    handle.join().unwrap()
    // Vec::new()
}
