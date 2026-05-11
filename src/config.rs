use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchDivergenceDisplay {
    Full,
    Compact,
    Minimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescribeStyle {
    Default,
    Contains,
    Branch,
    Describe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Posh,
    Plain,
    Nerd,
    Custom,
    PoshRounded,
    Emoji,
    Minimal,
    Retro,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UntrackedMode {
    No,
    Normal,
    All,
}

#[derive(Debug, Clone)]
pub struct Symbols {
    pub added: String,
    pub modified: String,
    pub removed: String,
    pub conflicted: String,
    pub local_clean: String,
    pub local_working: String,
    pub local_staged: String,
    pub branch_untracked: String,
    pub branch_gone: String,
    pub branch_identical: String,
    pub branch_ahead: String,
    pub branch_behind: String,
    pub branch_diverged: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub abbreviate_git_dir: bool,
    pub abbreviate_home: bool,
    pub before_stash: String,
    pub after_stash: String,
    pub before_status: String,
    pub after_status: String,
    pub branch_display: BranchDivergenceDisplay,
    pub branch_name_limit: usize,
    pub color_mode: ColorMode,
    pub delim_status: String,
    pub describe_style: DescribeStyle,
    pub disabled_repositories: Vec<String>,
    pub enable_file_status: bool,
    pub enable_prompt_status: bool,
    pub enable_stash_status: bool,
    pub path_status_separator: String,
    pub prompt_before_suffix: String,
    pub prompt_prefix: Option<String>,
    pub prompt_suffix: String,
    pub show_exit_status: bool,
    pub show_status_when_zero: bool,
    pub status_first: bool,
    pub symbols: Symbols,
    pub theme: Theme,
    pub truncated_branch_suffix: String,
    pub untracked_mode: UntrackedMode,
}

impl Default for Symbols {
    fn default() -> Self {
        Self {
            added: "+".to_string(),
            modified: "~".to_string(),
            removed: "-".to_string(),
            conflicted: "!".to_string(),
            local_clean: String::new(),
            local_working: "!".to_string(),
            local_staged: "~".to_string(),
            branch_untracked: String::new(),
            branch_gone: "×".to_string(),
            branch_identical: "≡".to_string(),
            branch_ahead: "↑".to_string(),
            branch_behind: "↓".to_string(),
            branch_diverged: "↕".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            abbreviate_git_dir: false,
            abbreviate_home: true,
            before_stash: " (".to_string(),
            after_stash: ")".to_string(),
            before_status: "(".to_string(),
            after_status: ")".to_string(),
            branch_display: BranchDivergenceDisplay::Full,
            branch_name_limit: 0,
            color_mode: ColorMode::Auto,
            delim_status: " |".to_string(),
            describe_style: DescribeStyle::Default,
            disabled_repositories: Vec::new(),
            enable_file_status: true,
            enable_prompt_status: true,
            enable_stash_status: false,
            path_status_separator: " ".to_string(),
            prompt_before_suffix: String::new(),
            prompt_prefix: None,
            prompt_suffix: "> ".to_string(),
            show_exit_status: false,
            show_status_when_zero: true,
            status_first: false,
            symbols: Symbols::default(),
            theme: Theme::PoshRounded,
            truncated_branch_suffix: "...".to_string(),
            untracked_mode: UntrackedMode::Normal,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let mut config = Self::default();

        if let Some(path) = config_path()
            && let Ok(contents) = fs::read_to_string(path)
        {
            config.apply_key_value_lines(&contents);
        }

        config.apply_env();
        config
    }

    pub fn color_enabled(&self) -> bool {
        match self.color_mode {
            ColorMode::Always => true,
            ColorMode::Never => false,
            ColorMode::Auto => {
                env::var_os("NO_COLOR").is_none()
                    && env::var("TERM").map(|term| term != "dumb").unwrap_or(true)
            }
        }
    }

    pub fn to_active_config_text(&self) -> String {
        let theme = match self.theme {
            Theme::Posh => "posh",
            Theme::Plain => "plain",
            Theme::Nerd => "nerd",
            Theme::Custom => "custom",
            Theme::PoshRounded => "posh-rounded",
            Theme::Emoji => "emoji",
            Theme::Minimal => "minimal",
            Theme::Retro => "retro",
        };
        let color = match self.color_mode {
            ColorMode::Auto => "auto",
            ColorMode::Always => "always",
            ColorMode::Never => "never",
        };
        let branch_display = match self.branch_display {
            BranchDivergenceDisplay::Full => "full",
            BranchDivergenceDisplay::Compact => "compact",
            BranchDivergenceDisplay::Minimal => "minimal",
        };
        let untracked = match self.untracked_mode {
            UntrackedMode::No => "no",
            UntrackedMode::Normal => "normal",
            UntrackedMode::All => "all",
        };
        let mut t = String::new();
        t.push_str("# theme: posh, plain, nerd, custom, posh-rounded, emoji, minimal, retro\n");
        t.push_str(&format!("theme={theme}\n"));
        t.push_str("# color: auto, always, never\n");
        t.push_str(&format!("color={color}\n"));
        t.push_str("# true, false\n");
        t.push_str(&format!(
            "enable_prompt_status={}\n",
            self.enable_prompt_status
        ));
        t.push_str(&format!("enable_file_status={}\n", self.enable_file_status));
        t.push_str(&format!(
            "enable_stash_status={}\n",
            self.enable_stash_status
        ));
        t.push_str("# untracked_files: no, normal, all\n");
        t.push_str(&format!("untracked_files={untracked}\n"));
        t.push_str(&format!(
            "show_zero_counts={}\n",
            self.show_status_when_zero
        ));
        t.push_str(&format!("status_first={}\n", self.status_first));
        t.push_str(&format!("abbreviate_home={}\n", self.abbreviate_home));
        t.push_str(&format!("abbreviate_git_dir={}\n", self.abbreviate_git_dir));
        t.push_str("# branch_display: full, compact, minimal\n");
        t.push_str(&format!("branch_display={branch_display}\n"));
        t.push_str(&format!("branch_name_limit={}\n", self.branch_name_limit));
        t.push_str(&format!("prompt_suffix={}\n", self.prompt_suffix));
        if let Some(prefix) = &self.prompt_prefix {
            t.push_str(&format!("prompt_prefix={prefix}\n"));
        }
        t.push_str(&format!(
            "path_status_separator={}\n",
            self.path_status_separator
        ));
        t.push_str(&format!("show_exit_status={}\n", self.show_exit_status));
        t.push_str("# symbols\n");
        t.push_str(&format!("symbol_added={}\n", self.symbols.added));
        t.push_str(&format!("symbol_modified={}\n", self.symbols.modified));
        t.push_str(&format!("symbol_removed={}\n", self.symbols.removed));
        t.push_str(&format!("symbol_conflicted={}\n", self.symbols.conflicted));
        t.push_str(&format!("symbol_ahead={}\n", self.symbols.branch_ahead));
        t.push_str(&format!("symbol_behind={}\n", self.symbols.branch_behind));
        t.push_str(&format!(
            "symbol_identical={}\n",
            self.symbols.branch_identical
        ));
        t.push_str(&format!(
            "symbol_diverged={}\n",
            self.symbols.branch_diverged
        ));
        t.push_str(&format!("symbol_gone={}\n", self.symbols.branch_gone));
        t
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        let target = normalize_key(key);
        for line in self.to_active_config_text().lines() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                if normalize_key(k) == target {
                    return Some(v.to_string());
                }
            }
        }
        None
    }

    pub fn set_in_file(key: &str, value: &str) -> Result<PathBuf, String> {
        let path = config_path()
            .ok_or_else(|| "cannot determine config path: HOME not set".to_string())?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create config directory: {e}"))?;
        }

        let existing = if path.exists() {
            fs::read_to_string(&path).map_err(|e| format!("failed to read config file: {e}"))?
        } else {
            String::new()
        };

        let target_norm = normalize_key(key);
        let new_line = format!("{key}={value}");
        let mut replaced = false;

        let mut lines: Vec<String> = existing
            .lines()
            .map(|line| {
                let trimmed = line.trim();
                if !replaced && !trimmed.is_empty() && !trimmed.starts_with('#') {
                    if let Some((k, _)) = trimmed.split_once('=') {
                        if normalize_key(k.trim()) == target_norm {
                            replaced = true;
                            return new_line.clone();
                        }
                    }
                }
                line.to_string()
            })
            .collect();

        if !replaced {
            if !existing.is_empty() && !existing.ends_with('\n') {
                lines.push(String::new());
            }
            lines.push(new_line);
        }

        let mut content = lines.join("\n");
        if !content.ends_with('\n') {
            content.push('\n');
        }

        fs::write(&path, &content).map_err(|e| format!("failed to write config file: {e}"))?;
        Ok(path)
    }

    fn apply_env(&mut self) {
        self.apply_env_key("GITFLECT_THEME", "theme");
        self.apply_env_key("GITFLECT_COLOR", "color");
        self.apply_env_key("GITFLECT_ENABLE_PROMPT_STATUS", "enable_prompt_status");
        self.apply_env_key("GITFLECT_ENABLE_FILE_STATUS", "enable_file_status");
        self.apply_env_key("GITFLECT_ENABLE_STASH", "enable_stash_status");
        self.apply_env_key("GITFLECT_ENABLE_STASH_STATUS", "enable_stash_status");
        self.apply_env_key("GITFLECT_UNTRACKED_FILES", "untracked_files");
        self.apply_env_key("GITFLECT_SHOW_ZERO", "show_zero_counts");
        self.apply_env_key("GITFLECT_SHOW_ZERO_COUNTS", "show_zero_counts");
        self.apply_env_key("GITFLECT_STATUS_FIRST", "status_first");
        self.apply_env_key("GITFLECT_ABBREV_HOME", "abbreviate_home");
        self.apply_env_key("GITFLECT_ABBREVIATE_HOME", "abbreviate_home");
        self.apply_env_key("GITFLECT_ABBREV_REPO", "abbreviate_git_dir");
        self.apply_env_key("GITFLECT_ABBREVIATE_GIT_DIR", "abbreviate_git_dir");
        self.apply_env_key("GITFLECT_BRANCH_DISPLAY", "branch_display");
        self.apply_env_key("GITFLECT_BRANCH_NAME_LIMIT", "branch_name_limit");
        self.apply_env_key("GITFLECT_DESCRIBE_STYLE", "describe_style");
        self.apply_env_key("GITFLECT_PREFIX", "prompt_prefix");
        self.apply_env_key("GITFLECT_SUFFIX", "prompt_suffix");
        self.apply_env_key("GITFLECT_BEFORE_SUFFIX", "prompt_before_suffix");
        self.apply_env_key("GITFLECT_PATH_STATUS_SEPARATOR", "path_status_separator");
        self.apply_env_key("GITFLECT_SHOW_EXIT_STATUS", "show_exit_status");
        self.apply_env_key("GITFLECT_DISABLED_REPOSITORIES", "disabled_repositories");
        self.apply_env_key("GITFLECT_SYMBOL_ADDED", "symbol_added");
        self.apply_env_key("GITFLECT_SYMBOL_MODIFIED", "symbol_modified");
        self.apply_env_key("GITFLECT_SYMBOL_REMOVED", "symbol_removed");
        self.apply_env_key("GITFLECT_SYMBOL_CONFLICTED", "symbol_conflicted");
        self.apply_env_key("GITFLECT_SYMBOL_WORKING", "symbol_working");
        self.apply_env_key("GITFLECT_SYMBOL_STAGED", "symbol_staged");
        self.apply_env_key("GITFLECT_SYMBOL_CLEAN", "symbol_clean");
        self.apply_env_key("GITFLECT_SYMBOL_AHEAD", "symbol_ahead");
        self.apply_env_key("GITFLECT_SYMBOL_BEHIND", "symbol_behind");
        self.apply_env_key("GITFLECT_SYMBOL_DIVERGED", "symbol_diverged");
        self.apply_env_key("GITFLECT_SYMBOL_IDENTICAL", "symbol_identical");
        self.apply_env_key("GITFLECT_SYMBOL_GONE", "symbol_gone");
    }

    fn apply_env_key(&mut self, env_key: &str, config_key: &str) {
        if let Ok(value) = env::var(env_key) {
            self.apply_key_value(config_key, &value);
        }
    }

    pub fn apply_key_value_lines(&mut self, contents: &str) {
        for raw_line in contents.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            self.apply_key_value(key.trim(), trim_config_value(value.trim()));
        }
    }

    fn apply_key_value(&mut self, key: &str, value: &str) {
        match normalize_key(key).as_str() {
            "abbreviategitdir" | "abbreviaterepo" => {
                if let Some(value) = parse_bool(value) {
                    self.abbreviate_git_dir = value;
                }
            }
            "abbreviatehome" => {
                if let Some(value) = parse_bool(value) {
                    self.abbreviate_home = value;
                }
            }
            "afterstash" => self.after_stash = value.to_string(),
            "afterstatus" => self.after_status = value.to_string(),
            "beforestash" => self.before_stash = value.to_string(),
            "beforestatus" => self.before_status = value.to_string(),
            "branchdisplay" | "branchbehindandaheaddisplay" => {
                if let Some(value) = parse_branch_display(value) {
                    self.branch_display = value;
                }
            }
            "branchnamelimit" => {
                if let Ok(value) = value.parse() {
                    self.branch_name_limit = value;
                }
            }
            "color" | "colormode" => {
                if let Some(value) = parse_color_mode(value) {
                    self.color_mode = value;
                }
            }
            "delimstatus" => self.delim_status = value.to_string(),
            "describestyle" => {
                if let Some(value) = parse_describe_style(value) {
                    self.describe_style = value;
                }
            }
            "disabledrepositories" | "repositoriesinwhichtodisablefilestatus" => {
                self.disabled_repositories = value
                    .split([':', ';'])
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(ToOwned::to_owned)
                    .collect();
            }
            "enablefilestatus" => {
                if let Some(value) = parse_bool(value) {
                    self.enable_file_status = value;
                }
            }
            "enablepromptstatus" => {
                if let Some(value) = parse_bool(value) {
                    self.enable_prompt_status = value;
                }
            }
            "enablestash" | "enablestashstatus" => {
                if let Some(value) = parse_bool(value) {
                    self.enable_stash_status = value;
                }
            }
            "pathstatusseparator" => self.path_status_separator = value.to_string(),
            "promptbeforesuffix" | "defaultpromptbeforesuffix" => {
                self.prompt_before_suffix = value.to_string();
            }
            "promptprefix" | "defaultpromptprefix" => {
                self.prompt_prefix = if value.is_empty() {
                    None
                } else {
                    Some(value.to_string())
                };
            }
            "promptsuffix" | "defaultpromptsuffix" => self.prompt_suffix = value.to_string(),
            "showexitstatus" => {
                if let Some(value) = parse_bool(value) {
                    self.show_exit_status = value;
                }
            }
            "showstatuswhenzero" | "showzerocounts" => {
                if let Some(value) = parse_bool(value) {
                    self.show_status_when_zero = value;
                }
            }
            "statusfirst" | "defaultpromptwritestatusfirst" => {
                if let Some(value) = parse_bool(value) {
                    self.status_first = value;
                }
            }
            "symboladded" => self.symbols.added = value.to_string(),
            "symbolmodified" => self.symbols.modified = value.to_string(),
            "symbolremoved" => self.symbols.removed = value.to_string(),
            "symbolconflicted" => self.symbols.conflicted = value.to_string(),
            "symbolworking" => self.symbols.local_working = value.to_string(),
            "symbolstaged" => self.symbols.local_staged = value.to_string(),
            "symbolclean" => self.symbols.local_clean = value.to_string(),
            "symbolahead" => self.symbols.branch_ahead = value.to_string(),
            "symbolbehind" => self.symbols.branch_behind = value.to_string(),
            "symboldiverged" => self.symbols.branch_diverged = value.to_string(),
            "symbolidentical" => self.symbols.branch_identical = value.to_string(),
            "symbolgone" => self.symbols.branch_gone = value.to_string(),
            "theme" => {
                if let Some(value) = parse_theme(value) {
                    self.theme = value;
                    self.apply_theme();
                }
            }
            "truncatedbranchsuffix" => self.truncated_branch_suffix = value.to_string(),
            "untrackedfiles" | "untrackedfilesmode" => {
                if let Some(value) = parse_untracked_mode(value) {
                    self.untracked_mode = value;
                }
            }
            _ => {}
        }
    }

    fn apply_theme(&mut self) {
        match self.theme {
            Theme::Posh => {}
            Theme::Plain => {
                self.symbols.branch_gone = "gone".to_string();
                self.symbols.branch_identical = "=".to_string();
                self.symbols.branch_ahead = "ahead".to_string();
                self.symbols.branch_behind = "behind".to_string();
                self.symbols.branch_diverged = "<>".to_string();
            }
            Theme::Nerd => {
                self.symbols.branch_identical = "󰘬".to_string();
                self.symbols.branch_ahead = "󰁝".to_string();
                self.symbols.branch_behind = "󰁅".to_string();
                self.symbols.branch_diverged = "󰃻".to_string();
                self.symbols.local_working = "●".to_string();
                self.symbols.local_staged = "●".to_string();
            }
            Theme::Custom => {
                // Symbols are left as-is; they come from config file symbol_* keys or env vars.
            }
            Theme::PoshRounded => {
                // Same symbols as Posh, but wraps the status block in ( ) instead of [ ].
                self.before_status = "(".to_string();
                self.after_status = ")".to_string();
            }
            Theme::Emoji => {
                self.symbols.branch_ahead = "⬆".to_string();
                self.symbols.branch_behind = "⬇".to_string();
                self.symbols.branch_diverged = "⇅".to_string();
                self.symbols.branch_identical = "✔".to_string();
                self.symbols.branch_gone = "✘".to_string();
                self.symbols.added = "✚".to_string();
                self.symbols.modified = "✎".to_string();
                self.symbols.removed = "✖".to_string();
                self.symbols.conflicted = "⚡".to_string();
                self.symbols.local_working = "✎".to_string();
                self.symbols.local_staged = "◉".to_string();
                self.symbols.local_clean = "✔".to_string();
            }
            Theme::Minimal => {
                self.symbols.branch_ahead = "^".to_string();
                self.symbols.branch_behind = "v".to_string();
                self.symbols.branch_diverged = "x".to_string();
                self.symbols.branch_identical = "=".to_string();
                self.symbols.branch_gone = "~".to_string();
                self.symbols.added = "+".to_string();
                self.symbols.modified = "*".to_string();
                self.symbols.removed = "-".to_string();
                self.symbols.conflicted = "!".to_string();
                self.symbols.local_working = "*".to_string();
                self.symbols.local_staged = "+".to_string();
                self.symbols.local_clean = String::new();
            }
            Theme::Retro => {
                self.symbols.branch_ahead = ">>".to_string();
                self.symbols.branch_behind = "<<".to_string();
                self.symbols.branch_diverged = "><".to_string();
                self.symbols.branch_identical = "--".to_string();
                self.symbols.branch_gone = "!!".to_string();
                self.symbols.added = "[+]".to_string();
                self.symbols.modified = "[~]".to_string();
                self.symbols.removed = "[-]".to_string();
                self.symbols.conflicted = "[!]".to_string();
                self.symbols.local_working = "[!]".to_string();
                self.symbols.local_staged = "[~]".to_string();
                self.symbols.local_clean = String::new();
            }
        }
    }
}

pub fn theme_dir() -> Option<PathBuf> {
    if let Some(path) = env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(path).join("gitflect").join("themes"));
    }
    env::var_os("HOME").map(|home| {
        PathBuf::from(home)
            .join(".config")
            .join("gitflect")
            .join("themes")
    })
}

pub fn save_named_theme(name: &str, pairs: &[(&str, &str)]) -> Result<PathBuf, String> {
    let dir = theme_dir().ok_or_else(|| "cannot determine theme directory".to_string())?;
    fs::create_dir_all(&dir).map_err(|e| format!("failed to create themes directory: {e}"))?;
    let path = dir.join(format!("{name}.conf"));
    let mut content = String::new();
    for (key, value) in pairs {
        content.push_str(&format!("{key}={value}\n"));
    }
    fs::write(&path, &content).map_err(|e| format!("failed to write theme file: {e}"))?;
    Ok(path)
}

pub fn load_named_theme(name: &str) -> Result<Vec<(String, String)>, String> {
    let dir = theme_dir().ok_or_else(|| "cannot determine theme directory".to_string())?;
    let path = dir.join(format!("{name}.conf"));
    let content = fs::read_to_string(&path).map_err(|_| {
        format!("theme '{name}' not found — use 'gitflect theme saved' to list themes")
    })?;
    let mut pairs = Vec::new();
    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            pairs.push((k.to_string(), v.to_string()));
        }
    }
    Ok(pairs)
}

pub fn list_named_themes() -> Vec<String> {
    let Some(dir) = theme_dir() else {
        return Vec::new();
    };
    let Ok(entries) = fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let path = e.path();
            if path.extension().is_some_and(|ext| ext == "conf") {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .map(str::to_string)
            } else {
                None
            }
        })
        .collect();
    names.sort();
    names
}

pub fn config_path() -> Option<PathBuf> {
    if let Some(path) = env::var_os("GITFLECT_CONFIG") {
        return Some(PathBuf::from(path));
    }

    if let Some(path) = env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(path).join("gitflect").join("config"));
    }

    env::var_os("HOME").map(|home| {
        PathBuf::from(home)
            .join(".config")
            .join("gitflect")
            .join("config")
    })
}

fn normalize_key(key: &str) -> String {
    key.chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "yes" | "on" | "true" => Some(true),
        "0" | "no" | "off" | "false" => Some(false),
        _ => None,
    }
}

fn parse_branch_display(value: &str) -> Option<BranchDivergenceDisplay> {
    match value.trim().to_ascii_lowercase().as_str() {
        "full" => Some(BranchDivergenceDisplay::Full),
        "compact" => Some(BranchDivergenceDisplay::Compact),
        "minimal" | "minimum" => Some(BranchDivergenceDisplay::Minimal),
        _ => None,
    }
}

fn parse_color_mode(value: &str) -> Option<ColorMode> {
    match value.trim().to_ascii_lowercase().as_str() {
        "auto" => Some(ColorMode::Auto),
        "always" | "true" | "yes" | "on" => Some(ColorMode::Always),
        "never" | "false" | "no" | "off" => Some(ColorMode::Never),
        _ => None,
    }
}

fn parse_describe_style(value: &str) -> Option<DescribeStyle> {
    match value.trim().to_ascii_lowercase().as_str() {
        "" | "default" | "tag" => Some(DescribeStyle::Default),
        "contains" => Some(DescribeStyle::Contains),
        "branch" => Some(DescribeStyle::Branch),
        "describe" => Some(DescribeStyle::Describe),
        _ => None,
    }
}

fn parse_theme(value: &str) -> Option<Theme> {
    match value.trim().to_ascii_lowercase().as_str() {
        "posh" | "default" => Some(Theme::Posh),
        "plain" | "ascii" => Some(Theme::Plain),
        "nerd" | "nerdfont" | "nerd-font" => Some(Theme::Nerd),
        "custom" => Some(Theme::Custom),
        "posh-rounded" | "poshrounded" | "rounded" => Some(Theme::PoshRounded),
        "emoji" => Some(Theme::Emoji),
        "minimal" | "minimum" => Some(Theme::Minimal),
        "retro" | "classic" => Some(Theme::Retro),
        _ => None,
    }
}

fn parse_untracked_mode(value: &str) -> Option<UntrackedMode> {
    match value.trim().to_ascii_lowercase().as_str() {
        "no" | "none" | "off" => Some(UntrackedMode::No),
        "normal" | "default" => Some(UntrackedMode::Normal),
        "all" => Some(UntrackedMode::All),
        _ => None,
    }
}

fn trim_config_value(value: &str) -> &str {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .or_else(|| {
            value
                .strip_prefix('\'')
                .and_then(|value| value.strip_suffix('\''))
        })
        .unwrap_or(value)
}
