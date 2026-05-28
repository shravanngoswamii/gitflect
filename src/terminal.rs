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

#[cfg(unix)]
pub fn terminal_rows() -> usize {
    use std::os::raw::{c_int, c_ulong};

    #[repr(C)]
    struct Winsize {
        ws_row: u16,
        ws_col: u16,
        ws_xpixel: u16,
        ws_ypixel: u16,
    }

    unsafe extern "C" {
        fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    }

    #[cfg(target_os = "macos")]
    const TIOCGWINSZ: c_ulong = 0x40087468;
    #[cfg(not(target_os = "macos"))]
    const TIOCGWINSZ: c_ulong = 0x5413;

    if let Ok(tty) = std::fs::File::open("/dev/tty") {
        use std::os::unix::io::AsRawFd;
        let fd = tty.as_raw_fd();
        let mut ws = Winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        unsafe {
            if ioctl(fd, TIOCGWINSZ, &mut ws) == 0 && ws.ws_row > 0 {
                return ws.ws_row as usize;
            }
        }
    }

    stty_terminal_rows()
}

#[cfg(not(unix))]
pub fn terminal_rows() -> usize {
    stty_terminal_rows()
}

fn stty_terminal_rows() -> usize {
    let stdin = std::fs::File::open("/dev/tty")
        .map(std::process::Stdio::from)
        .unwrap_or_else(|_| std::process::Stdio::null());
    std::process::Command::new("stty")
        .arg("size")
        .stdin(stdin)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().split_once(' ').and_then(|(r, _)| r.parse().ok()))
        .unwrap_or(24)
}
