use crate::quantity::PixelDistance;
use crate::scanline::{Frame, ScanLine, ScanLineMem, Scaler};
use crate::gpio::{GPIO, Reading};
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicU32};

pub fn circular_cutoff(x: i32, y: i32, radius: PixelDistance) {
    use crate::mask::*;
    use image::Luma;
    use imageproc::geometric_transformations::translate;

    let mut call_number: u32 = 0;
    let mut mask = Mask::from_pixel(4320, 2560, Luma([255]));
    apply_circular_cutoff(&mut mask, radius);
    let shifted = translate(&mask, (x, y));
    println!("DEBUG: shifted size = {}", shifted.as_raw().len());
    crate::lcd::run(|dm| {
        println!("DEBUG: dm size = {}", dm.as_mut().len());
        dm.as_mut().copy_from_slice(shifted.as_raw());
        call_number += 1;
        call_number >= 200
    }).unwrap();
}

pub fn flicker() {
    let mut call_number: u32 = 0;
    crate::lcd::run(|dm| {
        dm.as_mut().fill(if (call_number % 10) > 5 {
            0u8
        } else {
            255u8
        });
        call_number += 1;
        call_number >= 300
    }).unwrap();
}

pub fn latency() -> u32 {
    let frame_counter = Arc::new(AtomicU32::new(0));
    let frame_counter_copy = frame_counter.clone();

    let handle = std::thread::spawn(move || {
        let slm = ScanLineMem::new().unwrap();
        let gpio = GPIO::new().unwrap();
        let mut photon_counts: Vec<((Frame, ScanLine), Reading)> = Vec::new();
        photon_counts.reserve(40000);
        for _ in 0 .. 20000 {
            let scanline = slm.read_scanline(Scaler::Scaler0);
            let frame = frame_counter_copy.load(Ordering::SeqCst);
            if let Some(pulses) = gpio.record_window() {
                photon_counts.push(((frame, scanline), pulses));
            }
        }
        return photon_counts;
    });

    let mut call_number: u32 = 0;
    let slm = crate::scanline::ScanLineMem::new().unwrap();
    let mut start_scanline = None;
    crate::lcd::run(|dm| {
        frame_counter.fetch_add(1, Ordering::SeqCst);
        // for pixel in dm.as_mut().iter_mut() {
        //     *pixel = if call_number % 3 == 0 {
        //         0u8
        //     } else if call_number % 3 == 1 {
        //         127u8
        //     } else if call_number % 3 == 2 {
        //         255u8
        //     } else {
        //         panic!("impossible")
        //     };
        // }
        if call_number == 20 {
            start_scanline = Some((frame_counter.load(Ordering::SeqCst),
                                   slm.read_scanline(Scaler::Scaler0)));
        }
        dm.as_mut().fill(if call_number < 20 {
            0u8
        } else {
            255u8
        });
        call_number += 1;
        call_number >= 300
    }).unwrap();

    let photon_counts = handle.join();

    let mut file = std::fs::File::create("/calibration.txt").unwrap();
    for tuple in photon_counts {
        file.write_all(format!("{:?}\n", tuple).as_bytes()).unwrap();
    }
    file.write_all(format!("{:?}\n", start_scanline.unwrap()).as_bytes()).unwrap();
    0
}
