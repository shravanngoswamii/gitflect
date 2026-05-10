use crate::config::Config;
use crate::terminal::{AltScreen, Key, RawMode, next_key, tty_read, tty_write};
use std::collections::HashMap;
use std::io::Write;

#[derive(Clone, Copy)]
enum SettingKind {
    Bool,
    Enum(&'static [&'static str]),
    Str,
    Uint,
}

struct SettingDef {
    key: &'static str,
    label: &'static str,
    section: &'static str,
    kind: SettingKind,
}

static SETTINGS: &[SettingDef] = &[
    SettingDef {
        key: "theme",
        label: "theme",
        section: "Theme",
        kind: SettingKind::Enum(&[
            "posh",
            "posh-rounded",
            "plain",
            "nerd",
            "emoji",
            "minimal",
            "retro",
            "custom",
        ]),
    },
    SettingDef {
        key: "enable_prompt_status",
        label: "enable_prompt_status",
        section: "Status",
        kind: SettingKind::Bool,
    },
    SettingDef {
        key: "enable_file_status",
        label: "enable_file_status",
        section: "Status",
        kind: SettingKind::Bool,
    },
    SettingDef {
        key: "enable_stash_status",
        label: "enable_stash_status",
        section: "Status",
        kind: SettingKind::Bool,
    },
    SettingDef {
        key: "untracked_files",
        label: "untracked_files",
        section: "Status",
        kind: SettingKind::Enum(&["no", "normal", "all"]),
    },
    SettingDef {
        key: "show_zero_counts",
        label: "show_zero_counts",
        section: "Status",
        kind: SettingKind::Bool,
    },
    SettingDef {
        key: "status_first",
        label: "status_first",
        section: "Status",
        kind: SettingKind::Bool,
    },
    SettingDef {
        key: "branch_display",
        label: "branch_display",
        section: "Branch",
        kind: SettingKind::Enum(&["full", "compact", "minimal"]),
    },
    SettingDef {
        key: "branch_name_limit",
        label: "branch_name_limit",
        section: "Branch",
        kind: SettingKind::Uint,
    },
    SettingDef {
        key: "prompt_suffix",
        label: "prompt_suffix",
        section: "Prompt",
        kind: SettingKind::Str,
    },
    SettingDef {
        key: "prompt_prefix",
        label: "prompt_prefix",
        section: "Prompt",
        kind: SettingKind::Str,
    },
    SettingDef {
        key: "path_status_separator",
        label: "path_status_separator",
        section: "Prompt",
        kind: SettingKind::Str,
    },
    SettingDef {
        key: "show_exit_status",
        label: "show_exit_status",
        section: "Prompt",
        kind: SettingKind::Bool,
    },
    SettingDef {
        key: "color",
        label: "color",
        section: "Appearance",
        kind: SettingKind::Enum(&["auto", "always", "never"]),
    },
    SettingDef {
        key: "abbreviate_home",
        label: "abbreviate_home",
        section: "Appearance",
        kind: SettingKind::Bool,
    },
    SettingDef {
        key: "abbreviate_git_dir",
        label: "abbreviate_git_dir",
        section: "Appearance",
        kind: SettingKind::Bool,
    },
];

enum Row {
    Section(&'static str),
    Setting(usize),
}

enum Mode {
    Browse,
    PickEnum(usize), // pick_cursor: index into enum options list
    EditText(String),
}

fn build_rows() -> Vec<Row> {
    let mut rows = Vec::new();
    let mut last = "";
    for (i, s) in SETTINGS.iter().enumerate() {
        if s.section != last {
            rows.push(Row::Section(s.section));
            last = s.section;
        }
        rows.push(Row::Setting(i));
    }
    rows
}

fn disp_value(config: &Config, pending: &HashMap<String, String>, key: &str) -> String {
    pending
        .get(key)
        .cloned()
        .or_else(|| config.get_value(key))
        .unwrap_or_default()
}

fn cycle(config: &Config, pending: &mut HashMap<String, String>, idx: usize, step: i32) {
    let s = &SETTINGS[idx];
    let cur = disp_value(config, pending, s.key);
    let new_val = match s.kind {
        SettingKind::Bool => if cur == "true" { "false" } else { "true" }.to_string(),
        SettingKind::Enum(opts) => {
            let pos = opts.iter().position(|&o| o == cur).unwrap_or(0);
            opts[((pos as i32 + step).rem_euclid(opts.len() as i32)) as usize].to_string()
        }
        _ => return,
    };
    pending.insert(s.key.to_string(), new_val);
}

fn adjust_scroll(rows: &[Row], current: usize, scroll: &mut usize, visible: usize) {
    let row_idx = rows
        .iter()
        .position(|r| matches!(r, Row::Setting(i) if *i == current))
        .unwrap_or(0);
    if row_idx < *scroll {
        *scroll = row_idx;
    } else if row_idx + 1 > *scroll + visible {
        *scroll = row_idx + 1 - visible;
    }
}

fn render(
    out: &mut impl Write,
    config: &Config,
    pending: &HashMap<String, String>,
    current: usize,
    rows: &[Row],
    scroll: usize,
    mode: &Mode,
    save_flash: bool,
    discard_warn: bool,
) {
    let label_w = SETTINGS.iter().map(|s| s.label.len()).max().unwrap_or(0);
    let visible = 20usize;

    let _ = write!(out, "\x1b[2J\x1b[H");
    let _ = write!(
        out,
        "\x1b[1m\x1b[96mgitflect\x1b[0m  Settings  \x1b[90m↑↓ navigate  ←→ / Enter cycle  s save  q quit\x1b[0m\r\n\r\n"
    );

    // In PickEnum mode, we expand the current setting row into multiple option rows.
    // We compute rows to display carefully.
    let pick_cursor = if let Mode::PickEnum(c) = mode {
        Some(*c)
    } else {
        None
    };

    for row in rows.iter().skip(scroll).take(visible) {
        match row {
            Row::Section(name) => {
                let _ = write!(out, "  \x1b[90m{}\x1b[0m\r\n", name.to_uppercase());
            }
            Row::Setting(i) => {
                let s = &SETTINGS[*i];
                let is_cur = *i == current;

                if is_cur {
                    if let Some(pc) = pick_cursor {
                        // PickEnum mode for this setting — show label row then all options
                        let _ = write!(out, "  \x1b[1m\x1b[96m▶  {:<label_w$}\x1b[0m\r\n", s.label);
                        if let SettingKind::Enum(opts) = s.kind {
                            let cur_val = disp_value(config, pending, s.key);
                            for (oi, opt) in opts.iter().enumerate() {
                                if oi == pc {
                                    let _ = write!(out, "\x1b[1m\x1b[96m  ▶ {}\x1b[0m\r\n", opt);
                                } else if *opt == cur_val {
                                    let _ = write!(out, "\x1b[32m  ✓ {}\x1b[0m\r\n", opt);
                                } else {
                                    let _ = write!(out, "\x1b[90m    {}\x1b[0m\r\n", opt);
                                }
                            }
                        }
                    } else {
                        // Browse mode or EditText mode for this setting
                        let value = match mode {
                            Mode::EditText(input) => format!("{input}_"),
                            _ => disp_value(config, pending, s.key),
                        };
                        let hint = match s.kind {
                            SettingKind::Bool => "  \x1b[90m←→\x1b[0m",
                            SettingKind::Enum(_) => "  \x1b[90m←→ / Enter expand\x1b[0m",
                            SettingKind::Str | SettingKind::Uint => {
                                if matches!(mode, Mode::EditText(_)) {
                                    "  \x1b[90mEnter confirm  Esc cancel\x1b[0m"
                                } else {
                                    "  \x1b[90mEnter to edit\x1b[0m"
                                }
                            }
                        };
                        let _ = write!(
                            out,
                            "  \x1b[1m\x1b[96m▶  {:<label_w$}\x1b[0m  \x1b[33m{}\x1b[0m{}\r\n",
                            s.label, value, hint
                        );
                    }
                } else {
                    let value = disp_value(config, pending, s.key);
                    let changed = pending.contains_key(s.key);
                    let color = if changed { "\x1b[33m" } else { "\x1b[32m" };
                    let _ = write!(
                        out,
                        "     {:<label_w$}   {}{}\x1b[0m\r\n",
                        s.label, color, value
                    );
                }
            }
        }
    }

    let _ = write!(out, "\r\n");
    match mode {
        Mode::PickEnum(_) => {
            let _ = write!(
                out,
                "  \x1b[90m↑↓ navigate  Space/Enter select  Esc cancel\x1b[0m\r\n"
            );
        }
        _ => {
            if discard_warn {
                let _ = write!(
                    out,
                    "  \x1b[31mUnsaved changes — press q again to discard, or s to save\x1b[0m\r\n"
                );
            } else if save_flash {
                let _ = write!(out, "  \x1b[32mSaved.\x1b[0m\r\n");
            } else if pending.is_empty() {
                let _ = write!(out, "  \x1b[90mNo unsaved changes\x1b[0m\r\n");
            } else {
                let n = pending.len();
                let _ = write!(
                    out,
                    "  \x1b[33m{} unsaved change{} — press s to save\x1b[0m\r\n",
                    n,
                    if n == 1 { "" } else { "s" }
                );
            }
        }
    }
    let _ = out.flush();
}

pub fn run_settings() {
    let Some(mut tty_w) = tty_write() else {
        eprintln!("cannot open /dev/tty");
        return;
    };
    let Some(mut tty_r) = tty_read() else {
        eprintln!("cannot open /dev/tty");
        return;
    };

    let _alt = AltScreen::enter(&mut tty_w);
    let Some(_raw) = RawMode::enter() else {
        eprintln!("cannot enter raw mode");
        return;
    };

    let _ = write!(tty_w, "\x1b[?1l");
    let _ = tty_w.flush();

    let config = Config::load();
    let rows = build_rows();
    let mut pending: HashMap<String, String> = HashMap::new();
    let mut current = 0usize;
    let mut scroll = 0usize;
    let mut mode = Mode::Browse;
    let mut save_flash = false;
    let mut discard_warn = false;

    render(
        &mut tty_w,
        &config,
        &pending,
        current,
        &rows,
        scroll,
        &mode,
        save_flash,
        discard_warn,
    );

    loop {
        let Some(key) = next_key(&mut tty_r) else {
            continue;
        };

        match mode {
            Mode::EditText(ref mut input) => {
                discard_warn = false;
                save_flash = false;
                match key {
                    Key::Enter => {
                        let val = input.clone();
                        pending.insert(SETTINGS[current].key.to_string(), val);
                        mode = Mode::Browse;
                    }
                    Key::Esc => {
                        mode = Mode::Browse;
                    }
                    Key::Backspace => {
                        let mut chars: Vec<char> = input.chars().collect();
                        chars.pop();
                        *input = chars.into_iter().collect();
                    }
                    Key::Char(ch) => {
                        input.push_str(&ch);
                    }
                    _ => {}
                }
                render(
                    &mut tty_w,
                    &config,
                    &pending,
                    current,
                    &rows,
                    scroll,
                    &mode,
                    save_flash,
                    discard_warn,
                );
                continue;
            }

            Mode::PickEnum(ref mut pick_cursor) => {
                if let SettingKind::Enum(opts) = SETTINGS[current].kind {
                    match key {
                        Key::Up | Key::Left => {
                            *pick_cursor = pick_cursor.checked_sub(1).unwrap_or(opts.len() - 1);
                        }
                        Key::Down | Key::Right => {
                            *pick_cursor = (*pick_cursor + 1) % opts.len();
                        }
                        Key::Space | Key::Enter => {
                            let chosen = opts[*pick_cursor].to_string();
                            pending.insert(SETTINGS[current].key.to_string(), chosen);
                            mode = Mode::Browse;
                        }
                        Key::Esc => {
                            mode = Mode::Browse;
                        }
                        _ => {}
                    }
                } else {
                    mode = Mode::Browse;
                }
                render(
                    &mut tty_w,
                    &config,
                    &pending,
                    current,
                    &rows,
                    scroll,
                    &mode,
                    save_flash,
                    discard_warn,
                );
                continue;
            }

            Mode::Browse => {}
        }

        // Browse mode handling
        save_flash = false;

        match key {
            Key::Quit | Key::Esc => {
                if pending.is_empty() {
                    break;
                }
                if discard_warn {
                    break;
                }
                discard_warn = true;
            }
            other => {
                discard_warn = false;
                match other {
                    Key::Up => {
                        current = current.saturating_sub(1);
                        adjust_scroll(&rows, current, &mut scroll, 20);
                    }
                    Key::Down => {
                        if current + 1 < SETTINGS.len() {
                            current += 1;
                        }
                        adjust_scroll(&rows, current, &mut scroll, 20);
                    }
                    Key::Left => {
                        cycle(&config, &mut pending, current, -1);
                    }
                    Key::Right | Key::Space => {
                        cycle(&config, &mut pending, current, 1);
                    }
                    Key::Enter => match SETTINGS[current].kind {
                        SettingKind::Str | SettingKind::Uint => {
                            let v = disp_value(&config, &pending, SETTINGS[current].key);
                            mode = Mode::EditText(v);
                        }
                        SettingKind::Bool => {
                            cycle(&config, &mut pending, current, 1);
                        }
                        SettingKind::Enum(opts) => {
                            // Find current option index to start pick cursor there
                            let cur_val = disp_value(&config, &pending, SETTINGS[current].key);
                            let pc = opts.iter().position(|&o| o == cur_val).unwrap_or(0);
                            // Adjust scroll so current setting is near top
                            let row_idx = rows
                                .iter()
                                .position(|r| matches!(r, Row::Setting(i) if *i == current))
                                .unwrap_or(0);
                            scroll = row_idx.saturating_sub(1);
                            mode = Mode::PickEnum(pc);
                        }
                    },
                    Key::Char(ref s) if s.eq_ignore_ascii_case("s") => {
                        let mut err = false;
                        for (k, v) in &pending {
                            if let Err(e) = Config::set_in_file(k, v) {
                                eprintln!("{e}");
                                err = true;
                            }
                        }
                        if !err {
                            pending.clear();
                            save_flash = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        render(
            &mut tty_w,
            &config,
            &pending,
            current,
            &rows,
            scroll,
            &mode,
            save_flash,
            discard_warn,
        );
    }

    drop(_raw);
    drop(_alt);

    if !pending.is_empty() {
        println!("Cancelled — no changes saved.");
    }
}
