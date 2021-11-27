use std::net::{TcpStream};
use std::io::{Read, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::api::*;

pub fn parse_command(string: &str) -> Option<Request> {
    match string {
        "scanning_box" => Some(Request::TakePicture(TakePictureReq {
            masks: vec![
                MaskSeq::ScanningBox,
            ],
        })),
        "reboot" => Some(Request::Reboot),
        _ => None,
    }
}

pub fn main() {
    let mut rl = Editor::<()>::new();

    'try_connect: loop {
        std::thread::sleep(std::time::Duration::from_millis(2000));
        match TcpStream::connect("192.168.3.2:3333") {
            Ok(mut stream) => {
                println!("Successfully connected to server in port 3333");

                loop {
                    let readline = rl.readline("monocle> ");

                    let line;
                    match readline {
                        Ok(l) => {
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

                    println!("Sent message, awaiting reply...");

                    let response_size: u32;

                    {
                        let mut data = [0u8; 4];
                        match stream.read_exact(&mut data) {
                            Ok(_) => {
                                let mut cursor = std::io::Cursor::new(&data);
                                response_size =
                                    cursor.read_u32::<LittleEndian>().unwrap();
                            },
                            Err(e) => {
                                println!("Failed to receive : {}", e);
                                continue;
                            }
                        }
                    }

                    let mut response_data = Vec::new();
                    response_data.resize(response_size as usize, 0u8);

                    let response: Response;
                    match stream.read_exact(&mut response_data) {
                        Ok(_) => {
                            response =
                                serde_cbor::from_slice(&response_data).unwrap();
                            println!("{:?}", response);
                        },
                        Err(e) => {
                            println!("Failed to receive : {}", e);
                            continue;
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
