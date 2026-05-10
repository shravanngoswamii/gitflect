use std::io::{Read, Write};

pub struct RawMode {
    saved: String,
}

impl RawMode {
    pub fn enter() -> Option<Self> {
        let out = std::process::Command::new("stty")
            .arg("-g")
            .stdin(std::process::Stdio::inherit())
            .output()
            .ok()?;
        let saved = String::from_utf8(out.stdout).ok()?.trim().to_string();

        let ok = std::process::Command::new("stty")
            .args(["raw", "-echo", "min", "0", "time", "1"])
            .stdin(std::process::Stdio::inherit())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if ok { Some(RawMode { saved }) } else { None }
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let _ = std::process::Command::new("stty")
            .arg(&self.saved)
            .stdin(std::process::Stdio::inherit())
            .output();
    }
}

pub struct AltScreen;

impl AltScreen {
    pub fn enter(tty: &mut impl Write) -> Self {
        let _ = write!(tty, "\x1b[?1049h\x1b[H");
        let _ = tty.flush();
        AltScreen
    }
}

impl Drop for AltScreen {
    fn drop(&mut self) {
        if let Ok(mut tty) = std::fs::OpenOptions::new().write(true).open("/dev/tty") {
            let _ = write!(tty, "\x1b[?1049l");
            let _ = tty.flush();
        }
    }
}

#[derive(Debug)]
pub enum Key {
    Char(String),
    Enter,
    Backspace,
    Up,
    Down,
    Left,
    Right,
    Space,
    Esc,
    Quit,
}

pub fn read_byte(tty: &mut impl Read) -> Option<u8> {
    let mut b = [0u8; 1];
    loop {
        match tty.read(&mut b) {
            Ok(1) => return Some(b[0]),
            Ok(0) => {}
            _ => return None,
        }
    }
}

pub fn try_read_byte(tty: &mut impl Read) -> Option<u8> {
    let mut b = [0u8; 1];
    match tty.read(&mut b) {
        Ok(1) => Some(b[0]),
        _ => None,
    }
}

pub fn next_key(tty: &mut impl Read) -> Option<Key> {
    match read_byte(tty)? {
        b'\r' | b'\n' => Some(Key::Enter),
        b' ' => Some(Key::Space),
        0x7f | 0x08 => Some(Key::Backspace),
        0x03 | b'q' | b'Q' => Some(Key::Quit),
        0x1b => match try_read_byte(tty) {
            Some(b'[') | Some(b'O') => match try_read_byte(tty) {
                Some(b'A') => Some(Key::Up),
                Some(b'B') => Some(Key::Down),
                Some(b'C') => Some(Key::Right),
                Some(b'D') => Some(Key::Left),
                _ => None,
            },
            None => Some(Key::Esc),
            Some(_) => None,
        },
        ch if ch >= 0x20 => {
            let mut bytes = vec![ch];
            let extra = if ch >= 0xF0 {
                3
            } else if ch >= 0xE0 {
                2
            } else if ch >= 0xC0 {
                1
            } else {
                0
            };
            for _ in 0..extra {
                if let Some(b) = try_read_byte(tty) {
                    bytes.push(b);
                }
            }
            String::from_utf8(bytes).ok().map(Key::Char)
        }
        _ => None,
    }
}

pub fn tty_write() -> Option<std::fs::File> {
    std::fs::OpenOptions::new()
        .write(true)
        .open("/dev/tty")
        .ok()
}

pub fn tty_read() -> Option<std::fs::File> {
    std::fs::OpenOptions::new().read(true).open("/dev/tty").ok()
}
