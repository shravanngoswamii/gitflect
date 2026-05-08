use crate::config::{Config, DescribeStyle, UntrackedMode};
use std::env;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChangeSet {
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub deleted: Vec<String>,
    pub unmerged: Vec<String>,
}

impl ChangeSet {
    pub fn has_changes(&self) -> bool {
        self.total() > 0
    }

    pub fn paths(&self) -> Vec<String> {
        let mut paths = Vec::new();
        for path in self
            .added
            .iter()
            .chain(self.modified.iter())
            .chain(self.deleted.iter())
            .chain(self.unmerged.iter())
        {
            if !paths.contains(path) {
                paths.push(path.clone());
            }
        }
        paths
    }

    pub fn total(&self) -> usize {
        self.added.len() + self.modified.len() + self.deleted.len() + self.unmerged.len()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GitStatus {
    pub ahead_by: usize,
    pub bare: bool,
    pub behind_by: usize,
    pub branch: String,
    pub git_dir: PathBuf,
    pub has_untracked: bool,
    pub index: ChangeSet,
    pub inside_git_dir: bool,
    pub repo_name: String,
    pub stash_count: usize,
    pub upstream: Option<String>,
    pub upstream_gone: bool,
    pub working: ChangeSet,
    pub worktree: Option<PathBuf>,
}

impl GitStatus {
    pub fn has_index(&self) -> bool {
        self.index.has_changes()
    }

    pub fn has_working(&self) -> bool {
        self.working.has_changes()
    }

    pub fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"repo_name\":\"{}\",",
                "\"branch\":\"{}\",",
                "\"upstream\":{},",
                "\"upstream_gone\":{},",
                "\"ahead_by\":{},",
                "\"behind_by\":{},",
                "\"has_index\":{},",
                "\"has_working\":{},",
                "\"has_untracked\":{},",
                "\"stash_count\":{},",
                "\"git_dir\":\"{}\",",
                "\"worktree\":{},",
                "\"index\":{},",
                "\"working\":{}",
                "}}"
            ),
            json_escape(&self.repo_name),
            json_escape(&self.branch),
            json_option(self.upstream.as_deref()),
            self.upstream_gone,
            self.ahead_by,
            self.behind_by,
            self.has_index(),
            self.has_working(),
            self.has_untracked,
            self.stash_count,
            json_escape(&self.git_dir.display().to_string()),
            self.worktree
                .as_ref()
                .map(|path| format!("\"{}\"", json_escape(&path.display().to_string())))
                .unwrap_or_else(|| "null".to_string()),
            changes_json(&self.index),
            changes_json(&self.working)
        )
    }
}

#[derive(Debug)]
pub struct GitError {
    message: String,
}

impl GitError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for GitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(formatter)
    }
}

impl std::error::Error for GitError {}

#[derive(Debug, Clone)]
struct RepoInfo {
    bare: bool,
    git_dir: PathBuf,
    inside_git_dir: bool,
    worktree: Option<PathBuf>,
}

#[derive(Debug, Default)]
struct ParsedBranch {
    ahead_by: usize,
    behind_by: usize,
    branch: Option<String>,
    upstream: Option<String>,
    upstream_gone: bool,
}

pub fn collect(config: &Config) -> Result<Option<GitStatus>, GitError> {
    let Some(repo) = discover()? else {
        return Ok(None);
    };

    let mut status = GitStatus {
        bare: repo.bare,
        git_dir: repo.git_dir.clone(),
        inside_git_dir: repo.inside_git_dir,
        repo_name: repo_name(&repo),
        worktree: repo.worktree.clone(),
        ..GitStatus::default()
    };

    let mut parsed_branch = ParsedBranch::default();
    if config.enable_file_status
        && !repo.inside_git_dir
        && !repo.bare
        && !is_disabled_repository(config, repo.worktree.as_deref())
    {
        let output = run_git(&status_args(config))?;
        if output.success {
            parse_status_output(&output.stdout, &mut status, &mut parsed_branch);
        }
    }

    status.ahead_by = parsed_branch.ahead_by;
    status.behind_by = parsed_branch.behind_by;
    status.upstream = parsed_branch.upstream;
    status.upstream_gone = parsed_branch.upstream_gone;
    status.branch = resolve_branch(parsed_branch.branch.as_deref(), &repo, config);

    if config.enable_stash_status && !repo.inside_git_dir && !repo.bare {
        status.stash_count = stash_count()?;
    }

    Ok(Some(status))
}

pub fn current_repo_root() -> Option<PathBuf> {
    discover().ok().flatten().and_then(|repo| repo.worktree)
}

pub fn git_alias(alias: &str) -> Option<String> {
    let output = run_git(&["config", &format!("alias.{alias}")]).ok()?;
    if !output.success {
        return None;
    }
    output
        .stdout
        .lines()
        .next()
        .map(str::trim)
        .and_then(|value| {
            if value.is_empty() {
                None
            } else {
                Some(value.to_string())
            }
        })
}

pub fn git_alias_names(filter: &str) -> Vec<String> {
    let Ok(output) = run_git(&["config", "--get-regexp", "^alias\\."]) else {
        return Vec::new();
    };
    if !output.success {
        return Vec::new();
    }

    let mut aliases = Vec::new();
    for line in output.stdout.lines() {
        if let Some(rest) = line.strip_prefix("alias.")
            && let Some((alias, _)) = rest.split_once(' ')
            && alias.starts_with(filter)
            && !aliases.iter().any(|existing| existing == alias)
        {
            aliases.push(alias.to_string());
        }
    }
    aliases.sort();
    aliases
}

pub fn git_branches(filter: &str, include_head_refs: bool, prefix: &str) -> Vec<String> {
    let mut branches = Vec::new();
    collect_branch_output(&["branch", "--no-color"], filter, prefix, &mut branches);
    collect_branch_output(
        &["branch", "--no-color", "-r"],
        filter,
        prefix,
        &mut branches,
    );

    if include_head_refs {
        for head in ["HEAD", "FETCH_HEAD", "ORIG_HEAD", "MERGE_HEAD"] {
            if head.starts_with(filter) {
                branches.push(format!("{prefix}{head}"));
            }
        }
    }

    unique_sorted(branches)
}

pub fn git_remote_unique_branches(filter: &str) -> Vec<String> {
    let Ok(output) = run_git(&["branch", "--no-color", "-r"]) else {
        return Vec::new();
    };
    if !output.success {
        return Vec::new();
    }

    let mut names = Vec::new();
    for line in output.stdout.lines() {
        let trimmed = line.trim();
        if trimmed.contains(" -> ") {
            continue;
        }
        let Some((_, branch)) = trimmed.split_once('/') else {
            continue;
        };
        names.push(branch.to_string());
    }

    let mut unique = Vec::new();
    for name in &names {
        if name.starts_with(filter)
            && names.iter().filter(|candidate| *candidate == name).count() == 1
            && !unique.contains(name)
        {
            unique.push(name.clone());
        }
    }
    unique.sort();
    unique
}

pub fn git_remote_branches(
    remote: &str,
    ref_prefix: &str,
    filter: &str,
    force: &str,
) -> Vec<String> {
    let Ok(output) = run_git(&["branch", "--no-color", "-r"]) else {
        return Vec::new();
    };
    if !output.success {
        return Vec::new();
    }

    let remote_prefix = format!("{remote}/");
    output
        .stdout
        .lines()
        .filter_map(|line| {
            let branch = line.trim();
            if branch.contains(" -> ") {
                return None;
            }
            branch.strip_prefix(&remote_prefix)
        })
        .filter(|branch| branch.starts_with(filter))
        .map(|branch| format!("{force}{ref_prefix}{branch}"))
        .collect()
}

pub fn git_remotes(filter: &str) -> Vec<String> {
    let Ok(output) = run_git(&["remote"]) else {
        return Vec::new();
    };
    if !output.success {
        return Vec::new();
    }
    output
        .stdout
        .lines()
        .map(str::trim)
        .filter(|remote| remote.starts_with(filter))
        .map(ToOwned::to_owned)
        .collect()
}

pub fn git_stashes(filter: &str) -> Vec<String> {
    let Ok(output) = run_git(&["stash", "list"]) else {
        return Vec::new();
    };
    if !output.success {
        return Vec::new();
    }
    output
        .stdout
        .lines()
        .filter_map(|line| line.split_once(':').map(|(stash, _)| stash))
        .filter(|stash| stash.starts_with(filter))
        .map(ToOwned::to_owned)
        .collect()
}

pub fn git_tags(filter: &str, prefix: &str) -> Vec<String> {
    let Ok(output) = run_git(&["tag"]) else {
        return Vec::new();
    };
    if !output.success {
        return Vec::new();
    }
    output
        .stdout
        .lines()
        .map(str::trim)
        .filter(|tag| tag.starts_with(filter))
        .map(|tag| format!("{prefix}{tag}"))
        .collect()
}

fn collect_branch_output(args: &[&str], filter: &str, prefix: &str, branches: &mut Vec<String>) {
    let Ok(output) = run_git(args) else {
        return;
    };
    if !output.success {
        return;
    }
    for line in output.stdout.lines() {
        let trimmed = line.trim_start_matches(['*', '+', ' ']).trim();
        if trimmed.is_empty()
            || trimmed == "(no branch)"
            || trimmed.starts_with("(HEAD detached ")
            || trimmed.contains(" -> ")
        {
            continue;
        }
        if trimmed.starts_with(filter) {
            branches.push(format!("{prefix}{trimmed}"));
        }
    }
}

fn discover() -> Result<Option<RepoInfo>, GitError> {
    let git_dir = run_git(&["rev-parse", "--git-dir"])?;
    if !git_dir.success {
        return Ok(None);
    }

    let current_dir = env::current_dir()
        .map_err(|error| GitError::new(format!("failed to read current directory: {error}")))?;
    let git_dir = absolutize(current_dir.join(git_dir.stdout.trim()));

    let worktree = run_git(&["rev-parse", "--show-toplevel"])?;
    let worktree = if worktree.success {
        let path = worktree.stdout.trim();
        if path.is_empty() {
            None
        } else {
            Some(absolutize(PathBuf::from(path)))
        }
    } else {
        None
    };

    let inside_git_dir = run_git(&["rev-parse", "--is-inside-git-dir"])?
        .stdout
        .trim()
        == "true";
    let bare = run_git(&["rev-parse", "--is-bare-repository"])?
        .stdout
        .trim()
        == "true";

    Ok(Some(RepoInfo {
        bare,
        git_dir,
        inside_git_dir,
        worktree,
    }))
}

fn status_args(config: &Config) -> Vec<&'static str> {
    let untracked = match config.untracked_mode {
        UntrackedMode::No => "-uno",
        UntrackedMode::Normal => "-unormal",
        UntrackedMode::All => "-uall",
    };

    vec![
        "--no-optional-locks",
        "-c",
        "core.quotepath=false",
        "-c",
        "color.status=false",
        "status",
        untracked,
        "--porcelain=v1",
        "--branch",
    ]
}

fn parse_status_output(output: &str, status: &mut GitStatus, branch: &mut ParsedBranch) {
    for line in output.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            parse_branch_line(rest, branch);
            continue;
        }

        parse_file_status_line(line, status);
    }
}

fn parse_branch_line(line: &str, branch: &mut ParsedBranch) {
    if let Some(name) = line.strip_prefix("Initial commit on ") {
        branch.branch = Some(name.trim().to_string());
        return;
    }
    if let Some(name) = line.strip_prefix("No commits yet on ") {
        branch.branch = Some(name.trim().to_string());
        return;
    }

    let (head, metadata) = line.split_once(' ').unwrap_or((line, ""));
    let (local, upstream) = head.split_once("...").unwrap_or((head, ""));
    if !local.is_empty() {
        branch.branch = Some(local.to_string());
    }
    if !upstream.is_empty() {
        branch.upstream = Some(upstream.to_string());
    }

    if let Some(meta) = metadata
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        branch.ahead_by = parse_count_after(meta, "ahead ").unwrap_or(0);
        branch.behind_by = parse_count_after(meta, "behind ").unwrap_or(0);
        branch.upstream_gone = meta.contains("gone");
    }
}

fn parse_count_after(text: &str, marker: &str) -> Option<usize> {
    let (_, rest) = text.split_once(marker)?;
    let number = rest
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect::<String>();
    number.parse().ok()
}

fn parse_file_status_line(line: &str, status: &mut GitStatus) {
    if line.len() < 3 {
        return;
    }

    let mut chars = line.chars();
    let index = chars.next().unwrap_or(' ');
    let working = chars.next().unwrap_or(' ');
    let path = line.get(3..).unwrap_or("").trim();
    let path = strip_wrapping_quotes(path.split(" -> ").next().unwrap_or(path)).to_string();
    if path.is_empty() {
        return;
    }

    if is_unmerged(index, working) {
        if index != ' ' && index != '?' {
            status.index.unmerged.push(path.clone());
        }
        if working != ' ' && working != '?' {
            status.working.unmerged.push(path);
        }
        return;
    }

    match index {
        'A' => status.index.added.push(path.clone()),
        'M' | 'R' | 'C' => status.index.modified.push(path.clone()),
        'D' => status.index.deleted.push(path.clone()),
        'U' => status.index.unmerged.push(path.clone()),
        _ => {}
    }

    match working {
        '?' | 'A' => {
            status.has_untracked = true;
            status.working.added.push(path);
        }
        'M' => status.working.modified.push(path),
        'D' => status.working.deleted.push(path),
        'U' => status.working.unmerged.push(path),
        _ => {}
    }
}

fn is_unmerged(index: char, working: char) -> bool {
    matches!(
        (index, working),
        ('D', 'D') | ('A', 'U') | ('U', 'D') | ('U', 'A') | ('D', 'U') | ('A', 'A') | ('U', 'U')
    )
}

fn strip_wrapping_quotes(path: &str) -> &str {
    path.strip_prefix('"')
        .and_then(|path| path.strip_suffix('"'))
        .unwrap_or(path)
}

fn resolve_branch(parsed_branch: Option<&str>, repo: &RepoInfo, config: &Config) -> String {
    let (rebase_branch, suffix) = operation_state(&repo.git_dir);
    let mut branch = rebase_branch
        .or_else(|| {
            parsed_branch
                .filter(|branch| !is_detached_status_branch(branch))
                .map(ToOwned::to_owned)
        })
        .or_else(symbolic_branch)
        .unwrap_or_else(|| detached_name(config));

    if repo.inside_git_dir && !repo.bare {
        branch = "GIT_DIR!".to_string();
    }

    if repo.bare && !branch.starts_with("BARE:") {
        branch = format!("BARE:{branch}");
    }

    branch.push_str(&suffix);
    branch
}

fn is_detached_status_branch(branch: &str) -> bool {
    branch == "HEAD" || branch.starts_with("HEAD ")
}

fn operation_state(git_dir: &Path) -> (Option<String>, String) {
    let rebase_merge = git_dir.join("rebase-merge");
    if rebase_merge.is_dir() {
        let kind = if rebase_merge.join("interactive").exists() {
            "REBASE-i"
        } else {
            "REBASE-m"
        };
        let branch = read_trimmed(rebase_merge.join("head-name")).map(strip_refs_heads);
        let suffix = step_suffix(
            kind,
            read_trimmed(rebase_merge.join("msgnum")),
            read_trimmed(rebase_merge.join("end")),
        );
        return (branch, suffix);
    }

    let rebase_apply = git_dir.join("rebase-apply");
    if rebase_apply.is_dir() {
        let kind = if rebase_apply.join("rebasing").exists() {
            "REBASE"
        } else if rebase_apply.join("applying").exists() {
            "AM"
        } else {
            "AM/REBASE"
        };
        let suffix = step_suffix(
            kind,
            read_trimmed(rebase_apply.join("next")),
            read_trimmed(rebase_apply.join("last")),
        );
        return (None, suffix);
    }

    for (file, state) in [
        ("MERGE_HEAD", "MERGING"),
        ("CHERRY_PICK_HEAD", "CHERRY-PICKING"),
        ("REVERT_HEAD", "REVERTING"),
        ("BISECT_LOG", "BISECTING"),
    ] {
        if git_dir.join(file).exists() {
            return (None, format!("|{state}"));
        }
    }

    (None, String::new())
}

fn step_suffix(kind: &str, step: Option<String>, total: Option<String>) -> String {
    match (step, total) {
        (Some(step), Some(total)) if !step.is_empty() && !total.is_empty() => {
            format!("|{kind} {step}/{total}")
        }
        _ => format!("|{kind}"),
    }
}

fn symbolic_branch() -> Option<String> {
    let output = run_git(&[
        "--no-optional-locks",
        "symbolic-ref",
        "--quiet",
        "--short",
        "HEAD",
    ])
    .ok()?;
    if !output.success {
        return None;
    }
    output
        .stdout
        .lines()
        .next()
        .map(str::trim)
        .and_then(|branch| {
            if branch.is_empty() {
                None
            } else {
                Some(branch.to_string())
            }
        })
}

fn detached_name(config: &Config) -> String {
    let described = match config.describe_style {
        DescribeStyle::Contains => {
            first_success_line(&["--no-optional-locks", "describe", "--contains", "HEAD"])
        }
        DescribeStyle::Branch => first_success_line(&[
            "--no-optional-locks",
            "describe",
            "--contains",
            "--all",
            "HEAD",
        ]),
        DescribeStyle::Describe => first_success_line(&["--no-optional-locks", "describe", "HEAD"]),
        DescribeStyle::Default => {
            first_success_line(&["--no-optional-locks", "tag", "--points-at", "HEAD"])
        }
    };

    if let Some(name) = described {
        return format!("({name})");
    }

    first_success_line(&["--no-optional-locks", "rev-parse", "--short", "HEAD"])
        .map(|hash| format!("({hash})"))
        .unwrap_or_else(|| "(unknown)".to_string())
}

fn first_success_line(args: &[&str]) -> Option<String> {
    let output = run_git(args).ok()?;
    if !output.success {
        return None;
    }
    output
        .stdout
        .lines()
        .next()
        .map(str::trim)
        .and_then(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.to_string())
            }
        })
}

fn stash_count() -> Result<usize, GitError> {
    let output = run_git(&["--no-optional-locks", "stash", "list"])?;
    if output.success {
        Ok(output.stdout.lines().count())
    } else {
        Ok(0)
    }
}

fn status_to_string(status: std::process::ExitStatus) -> bool {
    status.success()
}

#[derive(Debug)]
struct GitOutput {
    stdout: String,
    success: bool,
}

fn run_git(args: &[&str]) -> Result<GitOutput, GitError> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|error| GitError::new(format!("failed to run git: {error}")))?;

    Ok(GitOutput {
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        success: status_to_string(output.status),
    })
}

fn repo_name(repo: &RepoInfo) -> String {
    repo.worktree
        .as_deref()
        .or(Some(repo.git_dir.as_path()))
        .and_then(Path::file_name)
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "repository".to_string())
}

fn read_trimmed(path: impl AsRef<Path>) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn strip_refs_heads(branch: String) -> String {
    branch
        .strip_prefix("refs/heads/")
        .unwrap_or(&branch)
        .to_string()
}

fn absolutize(path: PathBuf) -> PathBuf {
    path.canonicalize().unwrap_or(path)
}

fn is_disabled_repository(config: &Config, worktree: Option<&Path>) -> bool {
    let Ok(current_dir) = env::current_dir() else {
        return false;
    };
    let current_dir = current_dir.display().to_string();
    let worktree = worktree.map(|path| path.display().to_string());

    config.disabled_repositories.iter().any(|disabled| {
        current_dir.starts_with(disabled)
            || worktree
                .as_ref()
                .map(|worktree| worktree.starts_with(disabled))
                .unwrap_or(false)
    })
}

fn unique_sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

fn changes_json(changes: &ChangeSet) -> String {
    format!(
        "{{\"added\":{},\"modified\":{},\"deleted\":{},\"unmerged\":{}}}",
        json_array(&changes.added),
        json_array(&changes.modified),
        json_array(&changes.deleted),
        json_array(&changes.unmerged)
    )
}

fn json_array(values: &[String]) -> String {
    let values = values
        .iter()
        .map(|value| format!("\"{}\"", json_escape(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{values}]")
}

fn json_option(value: Option<&str>) -> String {
    value
        .map(|value| format!("\"{}\"", json_escape(value)))
        .unwrap_or_else(|| "null".to_string())
}

fn json_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_control() => {
                escaped.push_str(&format!("\\u{:04x}", character as u32));
            }
            character => escaped.push(character),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_mixed_status_counts() {
        let mut status = GitStatus::default();
        let mut branch = ParsedBranch::default();
        parse_status_output(
            concat!(
                "## master...origin/master [ahead 2, behind 1]\n",
                "A  staged-added\n",
                "D  staged-deleted\n",
                "R  old-name -> new-name\n",
                " M working-modified\n",
                " D working-deleted\n",
                "?? untracked\n",
            ),
            &mut status,
            &mut branch,
        );

        assert_eq!(branch.branch.as_deref(), Some("master"));
        assert_eq!(branch.upstream.as_deref(), Some("origin/master"));
        assert_eq!(branch.ahead_by, 2);
        assert_eq!(branch.behind_by, 1);
        assert_eq!(status.index.added, ["staged-added"]);
        assert_eq!(status.index.deleted, ["staged-deleted"]);
        assert_eq!(status.index.modified, ["old-name"]);
        assert_eq!(status.working.modified, ["working-modified"]);
        assert_eq!(status.working.deleted, ["working-deleted"]);
        assert_eq!(status.working.added, ["untracked"]);
        assert!(status.has_untracked);
    }

    #[test]
    fn parses_upstream_gone() {
        let mut branch = ParsedBranch::default();
        parse_branch_line("main...origin/main [gone]", &mut branch);
        assert_eq!(branch.branch.as_deref(), Some("main"));
        assert_eq!(branch.upstream.as_deref(), Some("origin/main"));
        assert!(branch.upstream_gone);
    }

    #[test]
    fn parses_initial_commit_branch() {
        let mut branch = ParsedBranch::default();
        parse_branch_line("Initial commit on trunk", &mut branch);
        assert_eq!(branch.branch.as_deref(), Some("trunk"));
    }
}
