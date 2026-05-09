mod complete;
mod config;
mod git;
mod render;
mod shell;

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
        "complete" => command_complete(&args[1..]),
        "config" => command_config(&args[1..]),
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

fn command_complete(args: &[String]) -> ExitCode {
    let config = Config::load();
    let mut position = None;
    let mut words = Vec::new();
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--position" => {
                let Some(value) = args.get(index + 1) else {
                    eprintln!("--position requires an integer");
                    return ExitCode::from(2);
                };
                position = value.parse::<usize>().ok();
                index += 2;
            }
            "--shell" => {
                if args.get(index + 1).is_none() {
                    eprintln!("--shell requires bash or zsh");
                    return ExitCode::from(2);
                }
                index += 2;
            }
            "--" => {
                words.extend(args[index + 1..].iter().cloned());
                break;
            }
            value => {
                words.push(value.to_string());
                index += 1;
            }
        }
    }

    let position = position.unwrap_or_else(|| words.len().saturating_sub(1));
    for completion in complete::complete(&words, position, &config) {
        println!("{completion}");
    }
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
            eprintln!("usage: gitflect config [path | init | default]");
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
  gitflect complete --position N -- WORDS...
  gitflect config                      Show active configuration
  gitflect config path                 Print config file path
  gitflect config init                 Create config file from defaults
  gitflect config default              Print default config template

SHELL SETUP:
  Bash: eval "$(gitflect init bash)"
  Zsh:  eval "$(gitflect init zsh)"

CONFIG:
  File: $GITFLECT_CONFIG or $XDG_CONFIG_HOME/gitflect/config or ~/.config/gitflect/config
  Env vars with GITFLECT_ prefix override the file (e.g. GITFLECT_ENABLE_STASH=true)
  Run 'gitflect config init' to create the file, 'gitflect config' to see active values.
"#,
        env!("CARGO_PKG_VERSION")
    );
}
