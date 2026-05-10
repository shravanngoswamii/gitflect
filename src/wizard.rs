use crate::terminal::{AltScreen, Key, RawMode, next_key, tty_read, tty_write};
use std::io::Write;

pub struct SymbolField {
    pub config_key: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub default: &'static str,
}

pub static SYMBOL_FIELDS: &[SymbolField] = &[
    SymbolField {
        config_key: "symbol_ahead",
        label: "branch_ahead",
        description: "commits ahead of upstream",
        default: "↑",
    },
    SymbolField {
        config_key: "symbol_behind",
        label: "branch_behind",
        description: "commits behind upstream",
        default: "↓",
    },
    SymbolField {
        config_key: "symbol_diverged",
        label: "branch_diverged",
        description: "both ahead and behind",
        default: "↕",
    },
    SymbolField {
        config_key: "symbol_identical",
        label: "branch_identical",
        description: "in sync with upstream",
        default: "≡",
    },
    SymbolField {
        config_key: "symbol_gone",
        label: "branch_gone",
        description: "upstream branch deleted",
        default: "×",
    },
    SymbolField {
        config_key: "symbol_added",
        label: "added",
        description: "new file staged",
        default: "+",
    },
    SymbolField {
        config_key: "symbol_modified",
        label: "modified",
        description: "file changed",
        default: "~",
    },
    SymbolField {
        config_key: "symbol_removed",
        label: "removed",
        description: "file deleted",
        default: "-",
    },
    SymbolField {
        config_key: "symbol_conflicted",
        label: "conflicted",
        description: "merge conflict",
        default: "!",
    },
    SymbolField {
        config_key: "symbol_working",
        label: "local_working",
        description: "unstaged changes indicator",
        default: "!",
    },
    SymbolField {
        config_key: "symbol_staged",
        label: "local_staged",
        description: "staged changes indicator",
        default: "~",
    },
    SymbolField {
        config_key: "symbol_clean",
        label: "local_clean",
        description: "nothing to commit (can be empty)",
        default: "",
    },
    SymbolField {
        config_key: "before_status",
        label: "bracket_open",
        description: "opens the status block (e.g. [ or ()",
        default: "[",
    },
    SymbolField {
        config_key: "after_status",
        label: "bracket_close",
        description: "closes the status block (e.g. ] or ))",
        default: "]",
    },
];

fn render_fields(out: &mut impl Write, values: &[String], current: usize, input: &str) {
    let label_w = SYMBOL_FIELDS
        .iter()
        .map(|f| f.label.len())
        .max()
        .unwrap_or(0);
    let desc_w = 34usize;
    let last = SYMBOL_FIELDS.len() - 1;

    let _ = write!(out, "\x1b[2J\x1b[H");
    let _ = write!(
        out,
        "\x1b[1m\x1b[96mgitflect\x1b[0m  Custom Theme Wizard  \x1b[90m({}/{})\x1b[0m\r\n",
        current + 1,
        SYMBOL_FIELDS.len()
    );
    let _ = write!(
        out,
        "\x1b[90m↑↓ navigate   Enter/↓ confirm & advance   q cancel\x1b[0m\r\n\r\n"
    );

    for (i, field) in SYMBOL_FIELDS.iter().enumerate() {
        let val = if i == current { input } else { &values[i] };
        let display = if val.is_empty() { field.default } else { val };

        if i == current {
            let _ = write!(
                out,
                "  \x1b[1m\x1b[96m▶  {:<label_w$}\x1b[0m  \x1b[90m{:<desc_w$}\x1b[0m  \x1b[33m[{}]\x1b[0m: {}_\r\n",
                field.label, field.description, field.default, input
            );
        } else {
            let color = if values[i].is_empty() {
                "\x1b[90m"
            } else {
                "\x1b[32m"
            };
            let _ = write!(
                out,
                "     {:<label_w$}   \x1b[90m{:<desc_w$}\x1b[0m  {}{}\x1b[0m\r\n",
                field.label, field.description, color, display
            );
        }
    }

    let _ = write!(out, "\r\n");
    if current == last {
        let _ = write!(
            out,
            "  \x1b[1m\x1b[32m► Last field — press Enter to review and save.\x1b[0m\r\n"
        );
    } else {
        let _ = write!(
            out,
            "  \x1b[90m{} more field{} — navigate with ↓ or Enter.\x1b[0m\r\n",
            last - current,
            if last - current == 1 { "" } else { "s" }
        );
    }
    let _ = out.flush();
}

fn render_summary(out: &mut impl Write, values: &[String]) {
    let label_w = SYMBOL_FIELDS
        .iter()
        .map(|f| f.label.len())
        .max()
        .unwrap_or(0);

    let _ = write!(out, "\x1b[2J\x1b[H");
    let _ = write!(
        out,
        "\x1b[1m\x1b[96mgitflect\x1b[0m  Custom Theme — Review\r\n\r\n"
    );

    for (field, value) in SYMBOL_FIELDS.iter().zip(values.iter()) {
        let display = if value.is_empty() {
            field.default
        } else {
            value.as_str()
        };
        let _ = write!(
            out,
            "   {:<label_w$}   \x1b[32m{}\x1b[0m\r\n",
            field.label, display
        );
    }

    let _ = write!(out, "\r\n");
    let _ = write!(
        out,
        "  Save to config? [\x1b[1mY\x1b[0m/n]   \x1b[90m↑ go back to edit\x1b[0m: "
    );
    let _ = out.flush();
}

fn render_theme_name_prompt(out: &mut impl Write, input: &str) {
    let _ = write!(out, "\x1b[2J\x1b[H");
    let _ = write!(
        out,
        "\x1b[1m\x1b[96mgitflect\x1b[0m  Save Named Theme\r\n\r\n"
    );
    let _ = write!(
        out,
        "  Give this theme a name to share it as a reusable file.\r\n"
    );
    let _ = write!(
        out,
        "  Named themes are saved to \x1b[90m~/.config/gitflect/themes/<name>.conf\x1b[0m\r\n\r\n"
    );
    let _ = write!(out, "  Name \x1b[90m(Enter to skip)\x1b[0m: {}_\r\n", input);
    let _ = out.flush();
}

pub struct WizardResult {
    pub pairs: Vec<(&'static str, String)>,
    pub theme_name: Option<String>,
}

pub fn run_wizard(config: &crate::config::Config) -> Option<WizardResult> {
    let mut tty_w = tty_write()?;
    let mut tty_r = tty_read()?;

    let _alt = AltScreen::enter(&mut tty_w);
    let _raw = RawMode::enter()?;

    let _ = write!(tty_w, "\x1b[?1l");
    let _ = tty_w.flush();

    let seed: Vec<String> = SYMBOL_FIELDS
        .iter()
        .map(|f| match f.config_key {
            "symbol_ahead" => config.symbols.branch_ahead.clone(),
            "symbol_behind" => config.symbols.branch_behind.clone(),
            "symbol_diverged" => config.symbols.branch_diverged.clone(),
            "symbol_identical" => config.symbols.branch_identical.clone(),
            "symbol_gone" => config.symbols.branch_gone.clone(),
            "symbol_added" => config.symbols.added.clone(),
            "symbol_modified" => config.symbols.modified.clone(),
            "symbol_removed" => config.symbols.removed.clone(),
            "symbol_conflicted" => config.symbols.conflicted.clone(),
            "symbol_working" => config.symbols.local_working.clone(),
            "symbol_staged" => config.symbols.local_staged.clone(),
            "symbol_clean" => config.symbols.local_clean.clone(),
            "before_status" => config.before_status.clone(),
            "after_status" => config.after_status.clone(),
            _ => String::new(),
        })
        .collect();

    let mut values = seed;
    let mut current = 0usize;
    let mut input = values[current].clone();

    render_fields(&mut tty_w, &values, current, &input);

    enum Phase {
        Fields,
        Summary,
    }
    let mut phase = Phase::Fields;

    let confirmed = loop {
        let Some(key) = next_key(&mut tty_r) else {
            continue;
        };

        match phase {
            Phase::Fields => match key {
                Key::Quit | Key::Esc => {
                    drop(_raw);
                    drop(_alt);
                    println!("Cancelled.");
                    return None;
                }
                Key::Backspace => {
                    let mut chars: Vec<char> = input.chars().collect();
                    chars.pop();
                    input = chars.into_iter().collect();
                    render_fields(&mut tty_w, &values, current, &input);
                }
                Key::Char(ch) => {
                    input.push_str(&ch);
                    render_fields(&mut tty_w, &values, current, &input);
                }
                Key::Enter | Key::Down => {
                    values[current] = if input.is_empty() {
                        SYMBOL_FIELDS[current].default.to_string()
                    } else {
                        input.clone()
                    };
                    if current + 1 < SYMBOL_FIELDS.len() {
                        current += 1;
                        input = values[current].clone();
                        render_fields(&mut tty_w, &values, current, &input);
                    } else {
                        phase = Phase::Summary;
                        render_summary(&mut tty_w, &values);
                    }
                }
                Key::Up => {
                    values[current] = if input.is_empty() {
                        SYMBOL_FIELDS[current].default.to_string()
                    } else {
                        input.clone()
                    };
                    if current > 0 {
                        current -= 1;
                        input = values[current].clone();
                        render_fields(&mut tty_w, &values, current, &input);
                    }
                }
                _ => {}
            },
            Phase::Summary => match key {
                Key::Enter => break true,
                Key::Char(ref s) if s.eq_ignore_ascii_case("y") => break true,
                Key::Char(ref s) if s.eq_ignore_ascii_case("n") => break false,
                Key::Quit => break false,
                Key::Esc | Key::Up => {
                    phase = Phase::Fields;
                    render_fields(&mut tty_w, &values, current, &input);
                }
                _ => {}
            },
        }
    };

    if !confirmed {
        drop(_raw);
        drop(_alt);
        println!("Cancelled.");
        return None;
    }

    // Named theme prompt — still in alt screen
    render_theme_name_prompt(&mut tty_w, "");
    let mut name_input = String::new();

    let theme_name: Option<String> = loop {
        let Some(key) = next_key(&mut tty_r) else {
            continue;
        };
        match key {
            Key::Enter => {
                break if name_input.trim().is_empty() {
                    None
                } else {
                    Some(name_input.trim().to_string())
                };
            }
            Key::Quit | Key::Esc => break None,
            Key::Backspace => {
                let mut chars: Vec<char> = name_input.chars().collect();
                chars.pop();
                name_input = chars.into_iter().collect();
                render_theme_name_prompt(&mut tty_w, &name_input);
            }
            Key::Char(ch) if !ch.contains('/') && !ch.contains('\\') => {
                name_input.push_str(&ch);
                render_theme_name_prompt(&mut tty_w, &name_input);
            }
            _ => {}
        }
    };

    drop(_raw);
    drop(_alt);

    let pairs = SYMBOL_FIELDS
        .iter()
        .zip(values)
        .map(|(f, v)| (f.config_key, v))
        .collect();
    Some(WizardResult { pairs, theme_name })
}
