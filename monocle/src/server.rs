use serde_derive::{Deserialize, Serialize};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;
use std::sync::{Arc, Mutex};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use once_cell::sync::Lazy;
use crate::api::*;

#[derive(Clone)]
struct TcpLogger {
    stream: Arc<Mutex<Option<TcpStream>>>,
}

impl log::Log for TcpLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        let s_record: SerializableRecord = SerializableRecord {
            args: record.args().to_string(),
            level: record.level() as usize,
            target: record.target().to_string(),
            module_path: record.module_path().map(|x| x.to_string())
                .or(record.module_path_static().map(|x| x.to_string())),
            file: record.file().map(|x| x.to_string())
                .or(record.file_static().map(|x| x.to_string())),
            line: record.line(),
        };
        let serialized =
            serde_cbor::to_vec(&Receivable::Event(Event::Log(s_record)))
            .unwrap();
        for _ in 0 .. 100 {
            if let Some(ref mut stream) = *self.stream.lock().unwrap() {
                stream.write_u32::<LittleEndian>(serialized.len() as u32).unwrap();
                stream.write_all(&serialized).unwrap();
                println!("printed log message!");
                break;
            } else {
                println!("failed to get lock");
            }
        }
    }

    fn flush(&self) {
        if let Some(ref mut stream) = *self.stream.lock().unwrap() {
            stream.flush().unwrap();
        }
    }
}

// static LOGGER: Lazy<TcpLogger> = Lazy::new(|| TcpLogger {
//     stream: Arc::new(Mutex::new(None)),
// });

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut data = [0 as u8; 4];
        stream.read_exact(&mut data).expect("Failed to receive length prefix");
        let mut cursor = std::io::Cursor::new(&data);
        let request_size = cursor.read_u32::<LittleEndian>().unwrap();

        let mut data = Vec::new();
        data.resize(request_size as usize, 0u8);
        stream.read_exact(&mut data).expect("Failed to receive message");

        let request: Request = serde_cbor::from_slice(&data).unwrap();

        // std::thread::sleep(std::time::Duration::from_millis(60000));

        let response = match request {
            Request::TakePicture(_) => {
                Response::TakePicture(TakePictureResp {
                    pulses: vec![crate::capture::scanbox()],
                })
            },
            Request::GoTo(_) => {
                Response::GoTo(GoToResp {})
            },
            Request::Command(req) => {
                let output = std::process::Command::new(req.command)
                    .args(req.arguments).output().unwrap();
                Response::Command(CommandResp {
                    status: output.status.code(),
                    stdout: output.stdout,
                    stderr: output.stderr,
                })
            },
            Request::Calibrate(CalibrateReq::Latency) => {
                std::thread::spawn(move || {
                    let _ = crate::calibrate::latency();
                });
                Response::Calibrate(CalibrateResp {})
            },
            Request::Calibrate(CalibrateReq::Flicker) => {
                // std::thread::spawn(move || {
                    let _ = crate::calibrate::flicker();
                // });
                Response::Calibrate(CalibrateResp {})
            },
            Request::Calibrate(CalibrateReq::Cutoff(x, y, dist)) => {
                std::thread::spawn(move || {
                    let _ = crate::calibrate::circular_cutoff(x, y, dist);
                });
                Response::Calibrate(CalibrateResp {})
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
            },
            Request::Reset => {
                use std::ffi::{CString, CStr};
                nix::unistd::close(stream.as_raw_fd()).unwrap();
                let serial = std::env::var("serial").unwrap();
                std::fs::remove_file("/bin/raspi").unwrap();
                let output = std::process::Command::new("atftp")
                    .args(&[
                        "-g",
                        "-r", &format!("{}/raspi", serial),
                        "-l", "/bin/raspi",
                        "192.168.3.1"
                    ])
                    .output().unwrap();
                if !output.status.success() {
                    println!("Failed to fetch new version of binary");
                    continue;
                }
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(
                    "/bin/raspi",
                    std::fs::Permissions::from_mode(0o777)).unwrap();
                std::thread::sleep(std::time::Duration::from_millis(250));
                let mut env: Vec<CString> = Vec::new();
                for (key, value) in std::env::vars() {
                    env.push(
                        CString::new(format!("{}={}", key, value)).unwrap());
                }
                let env_refs: Vec<&CStr> =
                    env.iter().map(|x| x.as_ref()).collect();
                nix::unistd::execve::<&CStr, &CStr>(
                    &CString::new("/bin/raspi").unwrap(),
                    &[], &env_refs).unwrap();
                panic!("This should never be reached")
            },
            Request::Close => {
                return;
            },
        };

        let serialized_response =
            serde_cbor::to_vec(&Receivable::Response(response)).unwrap();

        // loop {
        //     if let Some(ref mut stream) = *(*LOGGER.stream).lock().unwrap() {
        stream.write_u32::<LittleEndian>(
            serialized_response.len() as u32).unwrap();
        stream.write_all(&serialized_response).unwrap();
        //         break;
        //     }
        //     std::thread::sleep(std::time::Duration::from_millis(1));
        // }
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
    // std::thread::spawn(|| {
    //     let mut call_number: u32 = 0;
    //     // let slm = crate::scanline::ScanLineMem::new().unwrap();
    //     crate::lcd::run(|dm| {
    //         // for pixel in dm.as_mut().iter_mut() {
    //         //     *pixel = if call_number % 3 == 0 {
    //         //         0u8
    //         //     } else if call_number % 3 == 1 {
    //         //         127u8
    //         //     } else if call_number % 3 == 2 {
    //         //         255u8
    //         //     } else {
    //         //         panic!("impossible")
    //         //     };
    //         // }
    //         dm.as_mut().fill(if call_number / 100 == 0 {
    //             0u8
    //         } else if call_number / 100 == 1 {
    //             255u8
    //         } else {
    //             panic!("impossible")
    //         });
    //         call_number += 1;
    //         call_number >= 200
    //     }).unwrap();
    // });

    // {
    //     use crate::scanline::*;
    //     use std::collections::BTreeSet;
    //     let slm = ScanLineMem::new().unwrap();
    //     let mut vec: Vec<u32> = Vec::new();
    //     vec.reserve(1500000);
    //     for _ in 0 .. 1500000 {
    //         vec.push(slm.read_dispstat(Scaler::Scaler0) & 0b11_1111_1111_1111);
    //     }
    //     let set: BTreeSet<u32> = vec.iter().cloned().collect();
    //     let mut file = std::fs::File::create("/dispstat.txt").unwrap();
    //     file.write_all(format!("{:?}\n", set).as_bytes()).unwrap();
    // }

    {
        use crate::scanline::*;
        let slm = ScanLineMem::new().unwrap();
        println!("Scaler 0 control register: {}",
                 slm.read_dispctrl(Scaler::Scaler0));
        println!("Scaler 1 control register: {}",
                 slm.read_dispctrl(Scaler::Scaler1));
        println!("Scaler 2 control register: {}",
                 slm.read_dispctrl(Scaler::Scaler2));
    }

    // log::set_logger(&*LOGGER).unwrap();
    stderrlog::new()
        .verbosity(100)
        .color(stderrlog::ColorChoice::Always)
        .init().unwrap();

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
    let mut handle: Option<std::thread::JoinHandle<()>> = None;
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                // if let Some(h) = handle {
                //     println!("Waiting for previous connection to finish");
                //     h.join().unwrap();
                // }
                // *(*LOGGER.stream).lock().unwrap() =
                //     Some(stream.try_clone().unwrap());
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle = Some(std::thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                }));
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
}
