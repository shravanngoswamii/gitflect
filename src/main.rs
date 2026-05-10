mod config;
mod git;
mod render;
mod shell;
mod terminal;
mod tui;
mod wizard;

use config::{ColorMode, Config};
use render::PromptOptions;
use shell::Shell;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        print_help();
        return ExitCode::SUCCESS;
    }

    match args[0].as_str() {
        "prompt" => command_prompt(&args[1..]),
        "status" => command_status(&args[1..]),
        "init" => command_init(&args[1..]),
        "config" => command_config(&args[1..]),
        "theme" => command_theme(&args[1..]),
        "settings" => {
            tui::run_settings();
            ExitCode::SUCCESS
        }
        "help" | "-h" | "--help" => {
            print_help();
            ExitCode::SUCCESS
        }
        "version" | "-V" | "--version" => {
            println!("gitflect {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        command => {
            eprintln!("unknown command: {command}");
            eprintln!("run `gitflect help` for usage");
            ExitCode::from(2)
        }
    }
}

fn command_prompt(args: &[String]) -> ExitCode {
    let mut config = Config::load();
    let mut options = PromptOptions::default();
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--shell" => {
                let Some(value) = args.get(index + 1) else {
                    eprintln!("--shell requires bash, zsh, raw, or plain");
                    return ExitCode::from(2);
                };
                let Some(shell) = Shell::parse(value) else {
                    eprintln!("unsupported shell: {value}");
                    return ExitCode::from(2);
                };
                options.shell = shell;
                index += 2;
            }
            "--last-status" => {
                let Some(value) = args.get(index + 1) else {
                    eprintln!("--last-status requires an integer");
                    return ExitCode::from(2);
                };
                options.last_status = value.parse().ok();
                index += 2;
            }
            "--status-only" => {
                options.status_only = true;
                index += 1;
            }
            "--no-color" => {
                config.color_mode = ColorMode::Never;
                if options.shell == Shell::Raw {
                    options.shell = Shell::Plain;
                }
                index += 1;
            }
            "--color" => {
                config.color_mode = ColorMode::Always;
                index += 1;
            }
            unknown => {
                eprintln!("unknown prompt option: {unknown}");
                return ExitCode::from(2);
            }
        }
    }

    print!("{}", render::render_prompt(&config, &options));
    ExitCode::SUCCESS
}

fn command_status(args: &[String]) -> ExitCode {
    let mut config = Config::load();
    let mut json = false;
    let mut shell = Shell::Raw;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                json = true;
                index += 1;
            }
            "--shell" => {
                let Some(value) = args.get(index + 1) else {
                    eprintln!("--shell requires bash, zsh, raw, or plain");
                    return ExitCode::from(2);
                };
                let Some(parsed) = Shell::parse(value) else {
                    eprintln!("unsupported shell: {value}");
                    return ExitCode::from(2);
                };
                shell = parsed;
                index += 2;
            }
            "--no-color" => {
                config.color_mode = ColorMode::Never;
                shell = Shell::Plain;
                index += 1;
            }
            "--color" => {
                config.color_mode = ColorMode::Always;
                index += 1;
            }
            unknown => {
                eprintln!("unknown status option: {unknown}");
                return ExitCode::from(2);
            }
        }
    }

    match git::collect(&config) {
        Ok(Some(status)) if json => println!("{}", status.to_json()),
        Ok(Some(status)) => println!("{}", render::render_status(&config, &status, shell)),
        Ok(None) if json => println!("null"),
        Ok(None) => {}
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

fn command_init(args: &[String]) -> ExitCode {
    let Some(shell_name) = args.first() else {
        eprintln!("init requires a shell: bash or zsh");
        return ExitCode::from(2);
    };
    let Some(shell) = Shell::parse(shell_name) else {
        eprintln!("unsupported shell: {shell_name}");
        return ExitCode::from(2);
    };
    if !matches!(shell, Shell::Bash | Shell::Zsh) {
        eprintln!("init requires a shell: bash or zsh");
        return ExitCode::from(2);
    }

    let binary = env::current_exe()
        .ok()
        .and_then(|path| path.into_os_string().into_string().ok())
        .unwrap_or_else(|| "gitflect".to_string());
    print!("{}", shell::init(shell, &binary));
    ExitCode::SUCCESS
}

fn command_config(args: &[String]) -> ExitCode {
    let subcommand = args.first().map(String::as_str).unwrap_or("");

    match subcommand {
        "path" => {
            match config::config_path() {
                Some(path) => println!("{}", path.display()),
                None => eprintln!("cannot determine config path: HOME not set"),
            }
            ExitCode::SUCCESS
        }

        "init" => {
            let Some(path) = config::config_path() else {
                eprintln!("cannot determine config path: HOME not set");
                return ExitCode::FAILURE;
            };
            if path.exists() {
                println!("config file already exists: {}", path.display());
                return ExitCode::SUCCESS;
            }
            if let Some(parent) = path.parent() {
                if let Err(error) = std::fs::create_dir_all(parent) {
                    eprintln!("failed to create config directory: {error}");
                    return ExitCode::FAILURE;
                }
            }
            if let Err(error) = std::fs::write(&path, Config::default_config_text()) {
                eprintln!("failed to write config file: {error}");
                return ExitCode::FAILURE;
            }
            println!("created {}", path.display());
            ExitCode::SUCCESS
        }

        "default" | "--print-default" => {
            print!("{}", Config::default_config_text());
            ExitCode::SUCCESS
        }

        "get" => {
            let Some(key) = args.get(1) else {
                eprintln!("usage: gitflect config get <key>");
                return ExitCode::from(2);
            };
            if !Config::is_known_key(key) {
                eprintln!("unknown config key: {key}");
                eprintln!("run 'gitflect config' to see all keys");
                return ExitCode::from(2);
            }
            let config = Config::load();
            match config.get_value(key) {
                Some(v) => {
                    if let Some(opts) = Config::valid_values_for(key) {
                        println!("{v}  # {opts}");
                    } else {
                        println!("{v}");
                    }
                }
                None => {
                    eprintln!("unknown config key: {key}");
                    return ExitCode::from(2);
                }
            }
            ExitCode::SUCCESS
        }

        "set" => {
            let (Some(key), Some(value)) = (args.get(1), args.get(2)) else {
                eprintln!("usage: gitflect config set <key> <value>");
                return ExitCode::from(2);
            };
            if !Config::is_known_key(key) {
                eprintln!("unknown config key: {key}");
                if let Some(opts) = Config::valid_values_for(key) {
                    eprintln!("valid values: {opts}");
                }
                eprintln!("run 'gitflect config' to see all keys");
                return ExitCode::from(2);
            }
            if let Some(opts) = Config::valid_values_for(key) {
                let normalized = value.trim().to_ascii_lowercase();
                let valid = opts.split(", ").any(|opt| opt == normalized);
                if !valid {
                    eprintln!("invalid value for {key}: {value}");
                    eprintln!("valid values: {opts}");
                    return ExitCode::from(2);
                }
            }
            match Config::set_in_file(key, value) {
                Ok(path) => println!("set {key}={value} in {}", path.display()),
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::FAILURE;
                }
            }
            ExitCode::SUCCESS
        }

        "" => {
            let config = Config::load();
            let path = config::config_path();
            match &path {
                Some(p) if p.exists() => println!("# config file: {}", p.display()),
                Some(p) => println!(
                    "# config file: {} (not found — using defaults)",
                    p.display()
                ),
                None => println!("# config file: unknown (HOME not set)"),
            }
            println!("# environment overrides take precedence over the file\n");
            print!("{}", config.to_active_config_text());
            ExitCode::SUCCESS
        }

        unknown => {
            eprintln!("unknown config subcommand: {unknown}");
            eprintln!(
                "usage: gitflect config [get <key> | set <key> <value> | path | init | default]"
            );
            ExitCode::from(2)
        }
    }
}

fn command_theme(args: &[String]) -> ExitCode {
    let subcommand = args.first().map(String::as_str).unwrap_or("list");

    match subcommand {
        "list" => {
            let cfg = Config::load();
            let active = match cfg.theme {
                config::Theme::Posh => "posh",
                config::Theme::PoshRounded => "posh-rounded",
                config::Theme::Plain => "plain",
                config::Theme::Nerd => "nerd",
                config::Theme::Emoji => "emoji",
                config::Theme::Minimal => "minimal",
                config::Theme::Retro => "retro",
                config::Theme::Custom => "custom",
            };
            let themes: &[(&str, &str)] = &[
                ("posh", "Unicode symbols, default (↑ ↓ ↕ ≡ ×)"),
                (
                    "posh-rounded",
                    "Like posh but with ( ) status brackets instead of [ ]",
                ),
                ("plain", "ASCII text labels (ahead, behind, <>, =, gone)"),
                ("nerd", "Nerd Font glyph icons (requires patched font)"),
                ("emoji", "Single-width Unicode symbol set (⬆ ⬇ ⇅ ✔ ✘ ✚ ✎ ✖)"),
                ("minimal", "Single ASCII char per segment (^ v x = ~ + * -)"),
                ("retro", "Bracket-style labels (>> << >< -- !! [+] [~] [-])"),
                ("custom", "Your own symbols  →  gitflect theme set custom"),
            ];
            println!("Available themes:");
            for (name, desc) in themes {
                if *name == active {
                    println!("\x1b[32m* {:<13} — {}\x1b[0m", name, desc);
                } else {
                    println!("\x1b[90m  {:<13}   {}\x1b[0m", name, desc);
                }
            }
            println!();
            println!("Use 'gitflect settings' to configure interactively.");
            ExitCode::SUCCESS
        }

        "set" => {
            let Some(name) = args.get(1) else {
                eprintln!("usage: gitflect theme set <posh|plain|nerd|custom>");
                return ExitCode::from(2);
            };

            if name.eq_ignore_ascii_case("custom") {
                let config = Config::load();
                match wizard::run_wizard(&config) {
                    None => ExitCode::SUCCESS,
                    Some(result) => {
                        let config_path = match Config::set_in_file("theme", "custom") {
                            Ok(p) => p,
                            Err(e) => {
                                eprintln!("{e}");
                                return ExitCode::FAILURE;
                            }
                        };
                        for (key, value) in &result.pairs {
                            if let Err(e) = Config::set_in_file(key, value) {
                                eprintln!("{e}");
                                return ExitCode::FAILURE;
                            }
                        }
                        println!("Custom theme saved to {}", config_path.display());
                        if let Some(theme_name) = result.theme_name {
                            let pairs_refs: Vec<(&str, &str)> =
                                result.pairs.iter().map(|(k, v)| (*k, v.as_str())).collect();
                            match config::save_named_theme(&theme_name, &pairs_refs) {
                                Ok(path) => {
                                    println!("Theme '{}' saved to {}", theme_name, path.display())
                                }
                                Err(e) => eprintln!("Warning: could not save named theme: {e}"),
                            }
                        }
                        println!("Run `gitflect status` to preview.");
                        ExitCode::SUCCESS
                    }
                }
            } else {
                let normalized = name.trim().to_ascii_lowercase();
                if !matches!(
                    normalized.as_str(),
                    "posh" | "plain" | "nerd" | "posh-rounded" | "emoji" | "minimal" | "retro"
                ) {
                    eprintln!("unknown theme: {name}");
                    eprintln!(
                        "valid themes: posh, posh-rounded, plain, nerd, emoji, minimal, retro, custom"
                    );
                    return ExitCode::from(2);
                }
                match Config::set_in_file("theme", &normalized) {
                    Ok(path) => {
                        println!("theme={normalized} written to {}", path.display());
                        ExitCode::SUCCESS
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        ExitCode::FAILURE
                    }
                }
            }
        }

        "save" => {
            let Some(name) = args.get(1) else {
                eprintln!("usage: gitflect theme save <name>");
                return ExitCode::from(2);
            };
            if name.contains('/') || name.contains('\\') || name.contains('.') {
                eprintln!("invalid theme name: {name}");
                return ExitCode::from(2);
            }
            let cfg = Config::load();
            let pairs: Vec<(&str, &str)> = vec![
                ("symbol_ahead", cfg.symbols.branch_ahead.as_str()),
                ("symbol_behind", cfg.symbols.branch_behind.as_str()),
                ("symbol_diverged", cfg.symbols.branch_diverged.as_str()),
                ("symbol_identical", cfg.symbols.branch_identical.as_str()),
                ("symbol_gone", cfg.symbols.branch_gone.as_str()),
                ("symbol_added", cfg.symbols.added.as_str()),
                ("symbol_modified", cfg.symbols.modified.as_str()),
                ("symbol_removed", cfg.symbols.removed.as_str()),
                ("symbol_conflicted", cfg.symbols.conflicted.as_str()),
                ("symbol_working", cfg.symbols.local_working.as_str()),
                ("symbol_staged", cfg.symbols.local_staged.as_str()),
                ("symbol_clean", cfg.symbols.local_clean.as_str()),
                ("before_status", cfg.before_status.as_str()),
                ("after_status", cfg.after_status.as_str()),
            ];
            match config::save_named_theme(name, &pairs) {
                Ok(path) => {
                    println!("Theme '{}' saved to {}", name, path.display());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("{e}");
                    ExitCode::FAILURE
                }
            }
        }

        "load" => {
            let Some(name) = args.get(1) else {
                eprintln!("usage: gitflect theme load <name>");
                return ExitCode::from(2);
            };
            match config::load_named_theme(name) {
                Ok(pairs) => {
                    let config_path = match Config::set_in_file("theme", "custom") {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("{e}");
                            return ExitCode::FAILURE;
                        }
                    };
                    for (k, v) in &pairs {
                        if let Err(e) = Config::set_in_file(k, v) {
                            eprintln!("{e}");
                            return ExitCode::FAILURE;
                        }
                    }
                    println!(
                        "Loaded theme '{}' as custom to {}",
                        name,
                        config_path.display()
                    );
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("{e}");
                    ExitCode::FAILURE
                }
            }
        }

        "saved" => {
            let themes = config::list_named_themes();
            if themes.is_empty() {
                println!("No saved themes. Use 'gitflect theme set custom' to create one.");
            } else {
                let dir = config::theme_dir()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default();
                println!("Saved themes in {}:", dir);
                for name in &themes {
                    println!("  {name}");
                }
            }
            ExitCode::SUCCESS
        }

        unknown => {
            eprintln!("unknown theme subcommand: {unknown}");
            eprintln!("usage: gitflect theme list");
            eprintln!("       gitflect theme set <posh|plain|nerd|custom>");
            eprintln!("       gitflect theme save <name>");
            eprintln!("       gitflect theme load <name>");
            eprintln!("       gitflect theme saved");
            ExitCode::from(2)
        }
    }
}

fn print_help() {
    println!(
        r#"gitflect {}

USAGE:
  gitflect prompt [--shell bash|zsh|raw|plain] [--status-only] [--last-status N]
  gitflect status [--json] [--shell bash|zsh|raw|plain]
  gitflect init bash|zsh
  gitflect settings                    Interactive settings explorer (↑↓←→ navigate, s save)
  gitflect theme list                  List available themes
  gitflect theme set <name>            Switch theme (posh, plain, nerd, custom…)
  gitflect theme save <name>           Save current custom symbols as a named theme
  gitflect theme load <name>           Load a named theme as the active custom theme
  gitflect theme saved                 List all saved named themes
  gitflect config                      Show active configuration
  gitflect config get <key>            Print the value of a config key
  gitflect config set <key> <value>    Set a config key in the config file
  gitflect config path                 Print config file path
  gitflect config init                 Create config file from defaults
  gitflect config default              Print default config template

THEMES:
  posh          Unicode symbols — default (↑ ↓ ↕ ≡ ×)
  posh-rounded  Like posh but with ( ) brackets instead of [ ]
  plain         ASCII text labels (ahead, behind, <>, =, gone)
  nerd          Nerd Font glyph icons — requires patched font
  emoji         Single-width Unicode symbols (⬆ ⬇ ⇅ ✔ ✘ ✚ ✎ ✖)
  minimal       Single ASCII char per segment (^ v x = ~ + * -)
  retro         Bracket-style labels (>> << >< [+] [~] [-])
  custom        Your own symbols — run 'gitflect theme set custom' for an interactive wizard

SHELL SETUP (manual install only — the install script handles this automatically):
  Bash: eval "$(gitflect init bash)"
  Zsh:  eval "$(gitflect init zsh)"

CONFIG:
  File: $GITFLECT_CONFIG or $XDG_CONFIG_HOME/gitflect/config or ~/.config/gitflect/config
  Env vars with GITFLECT_ prefix override the file (e.g. GITFLECT_ENABLE_STASH=true)
  Run 'gitflect config init' to create the file, 'gitflect config' to see active values.
  Run 'gitflect config set theme plain' to change a setting from the command line.

BUGS:
  Report issues at: https://github.com/shravanngoswamii/gitflect/issues
  Security issues: https://github.com/shravanngoswamii/gitflect/security
"#,
        env!("CARGO_PKG_VERSION")
    );
}
