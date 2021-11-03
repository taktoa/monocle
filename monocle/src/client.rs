use std::collections::HashMap;
use std::error::Error;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use scan_fmt::scan_fmt;
use crate::api::*;

pub fn parse_command(string: &str) -> Option<Request> {
    // TODO: use scan_fmt for this parsing
    if string == "reset" {
        println!("\x1Bc\n");
        None
    } else if string == "scanning_box" {
        Some(Request::TakePicture(TakePictureReq {
            masks: vec![
                MaskSeq::ScanningBox,
            ],
        }))
    } else if string == "reboot" {
        Some(Request::Reboot)
    } else if string == "execve" {
        Some(Request::Reset)
    } else if string == "latency" {
        Some(Request::Calibrate(CalibrateReq::Latency))
    } else if string == "flicker" {
        Some(Request::Calibrate(CalibrateReq::Flicker))
    } else if let Some(args) = string.strip_prefix("cutoff ") {
        let (x, y, dist) = scan_fmt!(args, "{d} {d} {f}",
                                     i32, i32, f64).ok()?;
        Some(Request::Calibrate(CalibrateReq::Cutoff(x, y, dist)))
    } else if let Some(command_with_args) = string.strip_prefix("run ") {
        let mut iterator = command_with_args.split_whitespace();
        let command = iterator.next()?.to_string();
        let arguments: Vec<String> =
            iterator.map(|slice| String::from(slice)).collect();
        Some(Request::Command(CommandReq { command, arguments }))
    } else {
        None
    }
}

pub fn handle_receivable(receivable: &Receivable) -> bool {
    match receivable {
        Receivable::Response(Response::Command(cmd)) => {
            println!("\x1B[1mstatus: {:?}\x1B[0m", cmd.status);
            let mut stdout = String::new();
            for line in std::str::from_utf8(&cmd.stdout).unwrap().lines() {
                stdout.push_str(&format!("  {}\n", line));
            }
            let mut stderr = String::new();
            for line in std::str::from_utf8(&cmd.stderr).unwrap().lines() {
                stderr.push_str(&format!("  {}\n", line));
            }
            println!("\x1B[1mstdout:\x1B[0m\n{}", stdout);
            println!("\x1B[1mstderr:\x1B[0m\n{}", stderr);
            true
        }
        Receivable::Response(Response::TakePicture(pulse_sets)) => {
            use crate::scanline::{Frame, ScanLine};
            use crate::gpio::Reading;
            use float_ord::FloatOrd;
            use exr::prelude::f16;

            let resolution = 1500 / (crate::capture::DIVIDER as usize);

            let filename_prefix =
                format!("/home/remy/compressive-output/{}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));

            std::fs::write(format!("{}.cbor", filename_prefix),
                           serde_cbor::to_vec(&(resolution, pulse_sets))
                           .unwrap()).unwrap();

            let readings: &Vec<((Frame, ScanLine), Reading)> =
                &pulse_sets.pulses[0];
            let mut frame_map: HashMap<Frame, (u32, u32)> = HashMap::new();
            for ((frame, scanline), reading) in readings {
                if !frame_map.contains_key(frame) {
                    frame_map.insert(*frame, (0, 0));
                }
                frame_map.get_mut(frame).unwrap().0 += 1;
                frame_map.get_mut(frame).unwrap().1 += reading.counter;
            }
            let frame_map: HashMap<Frame, f32> =
                frame_map.iter().map(
                    |(f, (d, n))| (*f, (*n as f32) / (*d as f32)))
                .collect();
            let mut image = Vec::new();
            image.resize(resolution * resolution, 0.0);
            for i in 0 .. (image.len() as Frame) {
                if frame_map.contains_key(&i) {
                    image[i as usize] = frame_map[&i];
                }
            }

            let mut min_value: FloatOrd<f32> = FloatOrd(1000000000.0);
            let mut max_value: FloatOrd<f32> = FloatOrd(0.0);
            for value in image.iter() {
                min_value = std::cmp::min(min_value, FloatOrd(*value as f32));
                max_value = std::cmp::max(max_value, FloatOrd(*value as f32));
            }
            exr::prelude::write_rgba_file(
                format!("{}.exr", &filename_prefix),
                resolution, resolution,
                |x, y| {
                    let intensity = image[x + y * resolution];
                    let adjusted =
                        (intensity - min_value.0) / (max_value.0 - min_value.0);
                    (adjusted, adjusted, adjusted, f16::from_f32(1.0))
                }
            ).unwrap();
            println!("Minimum value: {}", min_value.0);
            println!("Maximum value: {}", max_value.0);
            println!("Wrote to {}.exr", filename_prefix);
            true
        },
        Receivable::Response(response) => {
            println!("{:?}", response);
            true
        },
        Receivable::Event(event) => {
            println!("{:?}", event);
            false
        }
    }
}

fn receive(stream: &mut TcpStream, timeout: Option<Duration>)
           -> Result<Receivable, Box<dyn Error>>
{
    let old_read_timeout = stream.read_timeout()?;

    stream.set_read_timeout(timeout)?;

    let receivable_size: u32;

    {
        let mut data = [0u8; 4];
        stream.read_exact(&mut data)
            .expect("Failed to receive length prefix");
        let mut cursor = std::io::Cursor::new(&data);
        receivable_size =
            cursor.read_u32::<LittleEndian>().unwrap();
    }

    let mut receivable_data = Vec::new();
    receivable_data.resize(receivable_size as usize, 0u8);

    stream.read_exact(&mut receivable_data)
        .expect("Failed to receive message");

    stream.set_read_timeout(old_read_timeout)?;

    Ok(serde_cbor::from_slice(&receivable_data)?)
}

pub fn main() {
    let mut rl = Editor::<()>::new();

    'try_connect: loop {
        std::thread::sleep(std::time::Duration::from_millis(2000));
        match TcpStream::connect("192.168.3.2:3333") {
            Ok(mut stream) => {
                println!("Successfully connected to server in port 3333");

                'repl: loop {
                    // let mut stream_copy = stream.try_clone().unwrap();
                    //
                    // let end_logger_bool =
                    //     Arc::new(std::sync::atomic::AtomicBool::new(false));
                    // let end_logger_clone = end_logger_bool.clone();
                    // let log_thread = std::thread::spawn(move || {
                    //     let mut vec = Vec::new();
                    //     while !end_logger_clone.load(Ordering::SeqCst) {
                    //         match receive(&mut stream_copy,
                    //                       Some(Duration::from_millis(20))) {
                    //             Ok(Receivable::Event(e)) => {
                    //                 vec.push(e);
                    //             },
                    //             _ => {},
                    //         }
                    //     }
                    //     vec
                    // });

                    let readline = rl.readline("monocle> ");

                    // end_logger_bool.store(true, Ordering::SeqCst);
                    // let log_vec = log_thread.join().unwrap();
                    // for log_line in log_vec {
                    //     println!("{:?}", log_line);
                    // }

                    let line;
                    match readline {
                        Ok(l) => {
                            if l.len() == 0 { continue; }
                            rl.add_history_entry(l.as_str());
                            line = l.clone();
                        },
                        Err(ReadlineError::Interrupted) => {
                            println!("CTRL-C");
                            break 'try_connect;
                        },
                        Err(ReadlineError::Eof) => {
                            println!("CTRL-D");
                            break 'try_connect;
                        },
                        Err(err) => {
                            println!("Error: {:?}", err);
                            break 'try_connect;
                        },
                    };

                    let request = parse_command(&line);
                    if request.is_none() {
                        println!("Failed to parse command: {:?}", line);
                        continue;
                    }
                    let request = request.unwrap();

                    let serialized = serde_cbor::to_vec(&request);
                    if let Err(_) = serialized {
                        println!("Failed to serialize command: {:?}", serialized);
                        continue;
                    }
                    let serialized = serialized.unwrap();

                    stream.write_u32::<LittleEndian>(
                        serialized.len() as u32).unwrap();
                    stream.write(&serialized).unwrap();

                    if let Request::Reboot = request {
                        println!("Rebooted remote server, dropping connection");
                        continue 'try_connect;
                    }

                    if let Request::Reset = request {
                        println!("Reset remote server, dropping connection");
                        continue 'try_connect;
                    }

                    println!("Sent message, awaiting reply...");

                    loop {
                        match receive(&mut stream, None) {
                            Ok(r) => {
                                if handle_receivable(&r) {
                                    break;
                                }
                            },
                            Err(e) => {
                                println!("Failed to receive: {}", e);
                                continue 'repl;
                            }
                        }
                    }
                }
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    }
}
