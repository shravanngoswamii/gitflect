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
            before_status: "[".to_string(),
            after_status: "]".to_string(),
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
            theme: Theme::Posh,
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

    pub fn default_config_text() -> &'static str {
        r#"# gitflect config
# Values are key=value. Environment variables with the GITFLECT_ prefix
# override this file.
theme=posh
color=auto
enable_prompt_status=true
enable_file_status=true
enable_stash_status=false
untracked_files=normal
show_zero_counts=true
status_first=false
abbreviate_home=true
abbreviate_git_dir=false
branch_display=full
branch_name_limit=0
prompt_suffix=> 
# prompt_prefix is empty by default. Over SSH, [user@host]: is shown automatically.
# prompt_prefix=
path_status_separator= 
"#
    }

    pub fn to_active_config_text(&self) -> String {
        let theme = match self.theme {
            Theme::Posh => "posh",
            Theme::Plain => "plain",
            Theme::Nerd => "nerd",
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
        let mut text = format!(
            "theme={theme}\n\
             color={color}\n\
             enable_prompt_status={}\n\
             enable_file_status={}\n\
             enable_stash_status={}\n\
             untracked_files={untracked}\n\
             show_zero_counts={}\n\
             status_first={}\n\
             abbreviate_home={}\n\
             abbreviate_git_dir={}\n\
             branch_display={branch_display}\n\
             branch_name_limit={}\n\
             prompt_suffix={}\n",
            self.enable_prompt_status,
            self.enable_file_status,
            self.enable_stash_status,
            self.show_status_when_zero,
            self.status_first,
            self.abbreviate_home,
            self.abbreviate_git_dir,
            self.branch_name_limit,
            self.prompt_suffix,
        );
        if let Some(prefix) = &self.prompt_prefix {
            text.push_str(&format!("prompt_prefix={prefix}\n"));
        }
        text.push_str(&format!(
            "path_status_separator={}\n\
             show_exit_status={}\n\
             symbol_added={}\n\
             symbol_modified={}\n\
             symbol_removed={}\n\
             symbol_conflicted={}\n\
             symbol_ahead={}\n\
             symbol_behind={}\n\
             symbol_identical={}\n\
             symbol_diverged={}\n\
             symbol_gone={}\n",
            self.path_status_separator,
            self.show_exit_status,
            self.symbols.added,
            self.symbols.modified,
            self.symbols.removed,
            self.symbols.conflicted,
            self.symbols.branch_ahead,
            self.symbols.branch_behind,
            self.symbols.branch_identical,
            self.symbols.branch_diverged,
            self.symbols.branch_gone,
        ));
        text
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

    fn apply_key_value_lines(&mut self, contents: &str) {
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
        }
    }
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
