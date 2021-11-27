use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::api::*;

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut data = [0 as u8; 4];
        stream.read_exact(&mut data).unwrap();
        let mut cursor = std::io::Cursor::new(&data);
        let request_size = cursor.read_u32::<LittleEndian>().unwrap();

        let mut data = Vec::new();
        data.resize(request_size as usize, 0u8);
        stream.read_exact(&mut data).unwrap();

        let request: Request = serde_cbor::from_slice(&data).unwrap();

        // std::thread::sleep(std::time::Duration::from_millis(60000));

        let response = match request {
            Request::TakePicture(_) => {
                Response::TakePicture(TakePictureResp {
                    pulses: vec![],
                })
            },
            Request::GoTo(_) => {
                Response::GoTo(GoToResp {})
            },
            Request::Reboot => {
                nix::unistd::close(stream.as_raw_fd()).unwrap();
                std::thread::sleep(std::time::Duration::from_millis(250));
                let mut sysrq = std::fs::OpenOptions::new()
                    .read(true).write(true)
                    .open("/proc/sysrq-trigger")
                    .unwrap();
                sysrq.write_all(b"b").unwrap();
                panic!("This should never be reached")
            }
        };

        let serialized_response = serde_cbor::to_vec(&response).unwrap();

        stream.write_u32::<LittleEndian>(
            serialized_response.len() as u32).unwrap();
        stream.write_all(&serialized_response).unwrap();
    }
    // while match stream.read(&mut data) {
    //     Ok(size) => {
    //         // echo everything!
    //         stream.write(&data[0..size]).unwrap();
    //         true
    //     },
    //     Err(_) => {
    //         println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
    //         stream.shutdown(Shutdown::Both).unwrap();
    //         false
    //     }
    // } {}
}

pub fn main() {
    std::thread::spawn(|| {
        let mut call_number: u32 = 0;
        // let slm = crate::scanline::ScanLineMem::new().unwrap();
        crate::lcd::run(|dm| {
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
            dm.as_mut().fill(if call_number / 100 == 0 {
                0u8
            } else if call_number / 100 == 1 {
                255u8
            } else {
                panic!("impossible")
            });
            call_number += 1;
            call_number >= 200
        }).unwrap();
    });

    // std::thread::spawn(move || {
    //     use crate::scanline::*;
    //     let slm = ScanLineMem::new().unwrap();
    //     for _ in 0 .. 200 {
    //         let mut vec = Vec::new();
    //         for _ in 0 .. 100 {
    //             vec.push(slm.read_scanline(Scaler::Scaler0).to_u32());
    //         }
    //         println!("DEBUG: {:?}", vec);
    //         std::thread::sleep(std::time::Duration::from_millis(500));
    //     }
    // });

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                std::thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
}
