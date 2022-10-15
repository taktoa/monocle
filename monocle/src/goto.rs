use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use scan_fmt::scan_fmt;
use crate::pantilt::*;
use crate::quantity::*;

pub fn parse_command(m: &mut HashMap<String, (Azimuth, Altitude)>,
                     conn: &mut Connection,
                     string: &str) {
    if let Some(target) = string.strip_prefix("goto ") {
        if let Some((az, alt)) = m.get(target) {
            conn.goto_az_alt(*az, *alt).unwrap();
        } else {
            println!("Couldn't find {}", target);
        }
    } else if let Some(name) = string.strip_prefix("record ") {
        let (az, alt) = conn.get_az_alt().unwrap();
        m.insert(name.to_string(), (az, alt));
    } else if let Some(name) = string.strip_prefix("print ") {
        if let Some((az, alt)) = m.get(name) {
            println!("{:?}", (az, alt));
        }
    }
}

pub fn main() {
    let mut conn = Connection::new().unwrap();
    // conn.port.write("e".as_bytes()).unwrap();
    // return;
    let mut rl = Editor::<()>::new();
    let mut m: HashMap<String, (Azimuth, Altitude)> = HashMap::new();

    loop {
        let readline = rl.readline("monocle> ");

        let line;
        match readline {
            Ok(l) => {
                if l.len() == 0 { continue; }
                rl.add_history_entry(l.as_str());
                line = l.clone();
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                return;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                return;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                return;
            },
        };

        parse_command(&mut m, &mut conn, &line);
    }
}
