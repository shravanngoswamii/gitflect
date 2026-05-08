use crate::config::{BranchDivergenceDisplay, ColorMode, Config};
use crate::git::{self, ChangeSet, GitStatus};
use crate::shell::Shell;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Branch,
    BranchAhead,
    BranchBehind,
    BranchDiverged,
    Delimiter,
    Error,
    Index,
    Stash,
    Working,
}

#[derive(Debug, Clone)]
pub struct PromptOptions {
    pub last_status: Option<i32>,
    pub shell: Shell,
    pub status_only: bool,
}

impl Default for PromptOptions {
    fn default() -> Self {
        Self {
            last_status: None,
            shell: Shell::Raw,
            status_only: false,
        }
    }
}

pub fn render_prompt(config: &Config, options: &PromptOptions) -> String {
    let status = if config.enable_prompt_status {
        git::collect(config).ok().flatten()
    } else {
        None
    };

    if options.status_only {
        return status
            .as_ref()
            .map(|status| render_status(config, status, options.shell))
            .unwrap_or_default();
    }

    let mut prompt = String::new();
    prompt.push_str(&text(&prompt_prefix(config), options.shell));

    let path = prompt_path(config, status.as_ref());
    let status_text = status
        .as_ref()
        .map(|status| render_status(config, status, options.shell))
        .filter(|value| !value.is_empty());

    if config.status_first {
        if let Some(status_text) = status_text.as_ref() {
            prompt.push_str(status_text);
            prompt.push_str(&text(&config.path_status_separator, options.shell));
        }
        prompt.push_str(&text(&path, options.shell));
    } else {
        prompt.push_str(&text(&path, options.shell));
        if let Some(status_text) = status_text.as_ref() {
            prompt.push_str(&text(&config.path_status_separator, options.shell));
            prompt.push_str(status_text);
        }
    }

    prompt.push_str(&text(&config.prompt_before_suffix, options.shell));

    if config.show_exit_status
        && let Some(code) = options.last_status.filter(|code| *code != 0)
    {
        prompt.push_str(&paint(
            config,
            &format!("({code}) "),
            Color::Error,
            options.shell,
        ));
    }

    prompt.push_str(&text(&config.prompt_suffix, options.shell));
    prompt
}

pub fn render_status(config: &Config, status: &GitStatus, shell: Shell) -> String {
    let mut output = String::new();
    output.push_str(&paint(
        config,
        &config.before_status,
        Color::Delimiter,
        shell,
    ));
    output.push_str(&paint(
        config,
        &format_branch_name(config, status),
        branch_color(status),
        shell,
    ));

    let branch_status = branch_status_text(config, status);
    if !branch_status.is_empty() {
        output.push(' ');
        output.push_str(&paint(config, &branch_status, branch_color(status), shell));
    }

    if config.enable_file_status && status.has_index() {
        output.push_str(&render_changes(
            config,
            &status.index,
            Color::Index,
            shell,
            false,
        ));

        if status.has_working() {
            output.push_str(&paint(
                config,
                &config.delim_status,
                Color::Delimiter,
                shell,
            ));
        }
    }

    if config.enable_file_status && status.has_working() {
        output.push_str(&render_changes(
            config,
            &status.working,
            Color::Working,
            shell,
            false,
        ));
    }

    let summary = local_status_symbol(config, status);
    if !summary.is_empty() {
        output.push(' ');
        output.push_str(&paint(config, summary, summary_color(status), shell));
    }

    if config.enable_stash_status && status.stash_count > 0 {
        output.push_str(&paint(config, &config.before_stash, Color::Stash, shell));
        output.push_str(&paint(
            config,
            &status.stash_count.to_string(),
            Color::Stash,
            shell,
        ));
        output.push_str(&paint(config, &config.after_stash, Color::Stash, shell));
    }

    output.push_str(&paint(
        config,
        &config.after_status,
        Color::Delimiter,
        shell,
    ));
    output
}

fn render_changes(
    config: &Config,
    changes: &ChangeSet,
    color: Color,
    shell: Shell,
    no_leading_space: bool,
) -> String {
    let mut output = String::new();
    let mut first = no_leading_space;
    append_count(
        &mut output,
        config,
        CountItem::new(&config.symbols.added, changes.added.len())
            .show(config.show_status_when_zero || !changes.added.is_empty()),
        color,
        shell,
        &mut first,
    );
    append_count(
        &mut output,
        config,
        CountItem::new(&config.symbols.modified, changes.modified.len())
            .show(config.show_status_when_zero || !changes.modified.is_empty()),
        color,
        shell,
        &mut first,
    );
    append_count(
        &mut output,
        config,
        CountItem::new(&config.symbols.removed, changes.deleted.len())
            .show(config.show_status_when_zero || !changes.deleted.is_empty()),
        color,
        shell,
        &mut first,
    );
    append_count(
        &mut output,
        config,
        CountItem::new(&config.symbols.conflicted, changes.unmerged.len())
            .show(!changes.unmerged.is_empty()),
        color,
        shell,
        &mut first,
    );
    output
}

#[derive(Debug, Clone, Copy)]
struct CountItem<'a> {
    count: usize,
    should_show: bool,
    symbol: &'a str,
}

impl<'a> CountItem<'a> {
    fn new(symbol: &'a str, count: usize) -> Self {
        Self {
            count,
            should_show: false,
            symbol,
        }
    }

    fn show(mut self, should_show: bool) -> Self {
        self.should_show = should_show;
        self
    }
}

fn append_count(
    output: &mut String,
    config: &Config,
    item: CountItem<'_>,
    color: Color,
    shell: Shell,
    no_leading_space: &mut bool,
) {
    if !item.should_show {
        return;
    }

    let prefix = if *no_leading_space {
        *no_leading_space = false;
        ""
    } else {
        " "
    };
    output.push_str(&paint(
        config,
        &format!("{prefix}{}{}", item.symbol, item.count),
        color,
        shell,
    ));
}

fn branch_status_text(config: &Config, status: &GitStatus) -> String {
    if status.upstream.is_none() {
        return config.symbols.branch_untracked.clone();
    }

    if status.upstream_gone {
        return config.symbols.branch_gone.clone();
    }

    match (status.behind_by, status.ahead_by) {
        (0, 0) => config.symbols.branch_identical.clone(),
        (behind, ahead) if behind > 0 && ahead > 0 => match config.branch_display {
            BranchDivergenceDisplay::Full => format!(
                "{}{} {}{}",
                config.symbols.branch_behind, behind, config.symbols.branch_ahead, ahead
            ),
            BranchDivergenceDisplay::Compact => {
                format!("{}{}{}", behind, config.symbols.branch_diverged, ahead)
            }
            BranchDivergenceDisplay::Minimal => config.symbols.branch_diverged.clone(),
        },
        (behind, 0) if behind > 0 => match config.branch_display {
            BranchDivergenceDisplay::Full | BranchDivergenceDisplay::Compact => {
                format!("{}{behind}", config.symbols.branch_behind)
            }
            BranchDivergenceDisplay::Minimal => config.symbols.branch_behind.clone(),
        },
        (0, ahead) if ahead > 0 => match config.branch_display {
            BranchDivergenceDisplay::Full | BranchDivergenceDisplay::Compact => {
                format!("{}{ahead}", config.symbols.branch_ahead)
            }
            BranchDivergenceDisplay::Minimal => config.symbols.branch_ahead.clone(),
        },
        _ => "?".to_string(),
    }
}

fn format_branch_name(config: &Config, status: &GitStatus) -> String {
    if config.branch_name_limit > 0 && status.branch.chars().count() > config.branch_name_limit {
        let prefix = status
            .branch
            .chars()
            .take(config.branch_name_limit)
            .collect::<String>();
        format!("{prefix}{}", config.truncated_branch_suffix)
    } else {
        status.branch.clone()
    }
}

fn branch_color(status: &GitStatus) -> Color {
    match (status.behind_by > 0, status.ahead_by > 0) {
        (true, true) => Color::BranchDiverged,
        (true, false) => Color::BranchBehind,
        (false, true) => Color::BranchAhead,
        (false, false) => Color::Branch,
    }
}

fn summary_color(status: &GitStatus) -> Color {
    if status.has_working() {
        Color::Working
    } else if status.has_index() {
        Color::Branch
    } else {
        Color::Index
    }
}

fn local_status_symbol<'a>(config: &'a Config, status: &GitStatus) -> &'a str {
    if status.has_working() {
        &config.symbols.local_working
    } else if status.has_index() {
        &config.symbols.local_staged
    } else {
        &config.symbols.local_clean
    }
}

fn prompt_path(config: &Config, status: Option<&GitStatus>) -> String {
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut display = current_dir.display().to_string();

    if config.abbreviate_git_dir {
        if let Some(worktree) = status.and_then(|status| status.worktree.as_deref()) {
            if let Some(path) = abbreviate_repo_path(&current_dir, worktree) {
                return path;
            }
        } else if let Some(worktree) = git::current_repo_root()
            && let Some(path) = abbreviate_repo_path(&current_dir, &worktree)
        {
            return path;
        }
    }

    if config.abbreviate_home
        && let Some(home) = env::var_os("HOME").map(PathBuf::from)
        && let Ok(relative) = current_dir.strip_prefix(&home)
    {
        display = if relative.as_os_str().is_empty() {
            "~".to_string()
        } else {
            format!("~/{}", relative.display())
        };
    }

    display
}

fn abbreviate_repo_path(current_dir: &Path, worktree: &Path) -> Option<String> {
    let repo_name = worktree.file_name()?.to_string_lossy();
    let relative = current_dir.strip_prefix(worktree).ok()?;
    if relative.as_os_str().is_empty() {
        Some(format!("{repo_name}:"))
    } else {
        Some(format!("{repo_name}:{}", relative.display()))
    }
}

fn prompt_prefix(config: &Config) -> String {
    if let Some(prefix) = &config.prompt_prefix {
        return prefix.clone();
    }

    if env::var_os("SSH_CONNECTION").is_some() {
        let user = env::var("USER").unwrap_or_else(|_| "user".to_string());
        let host = env::var("HOSTNAME")
            .ok()
            .filter(|host| !host.is_empty())
            .unwrap_or_else(|| "host".to_string());
        format!("[{user}@{host}]: ")
    } else {
        String::new()
    }
}

pub fn paint(config: &Config, value: &str, color: Color, shell: Shell) -> String {
    if value.is_empty() {
        return String::new();
    }

    if !config.color_enabled() || matches!(config.color_mode, ColorMode::Never) {
        return text(value, shell);
    }

    format!(
        "{}{}{}",
        ansi(shell, color_code(color)),
        text(value, shell),
        ansi(shell, "39")
    )
}

pub fn text(value: &str, shell: Shell) -> String {
    match shell {
        Shell::Bash => escape_bash_prompt_text(value),
        Shell::Zsh => escape_zsh_prompt_text(value),
        Shell::Raw | Shell::Plain => value.to_string(),
    }
}

fn ansi(shell: Shell, code: &str) -> String {
    match shell {
        Shell::Bash => format!("\\[\x1b[{code}m\\]"),
        Shell::Zsh => format!("%{{\x1b[{code}m%}}"),
        Shell::Raw => format!("\x1b[{code}m"),
        Shell::Plain => String::new(),
    }
}

fn color_code(color: Color) -> &'static str {
    match color {
        Color::Branch => "96",
        Color::BranchAhead => "32",
        Color::BranchBehind => "31",
        Color::BranchDiverged => "33",
        Color::Delimiter => "93",
        Color::Error => "31",
        Color::Index => "32",
        Color::Stash => "31",
        Color::Working => "31",
    }
}

fn escape_bash_prompt_text(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '$' => escaped.push_str("\\$"),
            '`' => escaped.push_str("\\`"),
            character => escaped.push(character),
        }
    }
    escaped
}

fn escape_zsh_prompt_text(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '%' => escaped.push_str("%%"),
            '$' => escaped.push_str("\\$"),
            '`' => escaped.push_str("\\`"),
            character => escaped.push(character),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    fn status() -> GitStatus {
        GitStatus {
            branch: "main".to_string(),
            upstream: Some("origin/main".to_string()),
            ahead_by: 0,
            behind_by: 0,
            index: ChangeSet {
                added: vec!["a".to_string()],
                ..ChangeSet::default()
            },
            working: ChangeSet {
                modified: vec!["b".to_string()],
                deleted: vec!["c".to_string()],
                ..ChangeSet::default()
            },
            ..GitStatus::default()
        }
    }

    #[test]
    fn renders_posh_style_status_without_color() {
        let config = Config {
            color_mode: ColorMode::Never,
            ..Config::default()
        };

        assert_eq!(
            render_status(&config, &status(), Shell::Plain),
            "[main ≡ +1 ~0 -0 | +0 ~1 -1 !]"
        );
    }

    #[test]
    fn renders_ahead_and_behind_full() {
        let config = Config {
            color_mode: ColorMode::Never,
            ..Config::default()
        };
        let status = GitStatus {
            branch: "main".to_string(),
            upstream: Some("origin/main".to_string()),
            ahead_by: 3,
            behind_by: 2,
            ..GitStatus::default()
        };

        assert_eq!(
            render_status(&config, &status, Shell::Plain),
            "[main ↓2 ↑3]"
        );
    }

    #[test]
    fn escapes_bash_prompt_expansions() {
        assert_eq!(
            text("feature/$thing`x`\\y", Shell::Bash),
            "feature/\\$thing\\`x\\`\\\\y"
        );
    }
}
