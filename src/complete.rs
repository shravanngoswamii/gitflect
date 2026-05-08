use crate::config::Config;
use crate::git::{self, ChangeSet};

const COMMANDS: &[&str] = &[
    "add",
    "am",
    "annotate",
    "archive",
    "bisect",
    "blame",
    "branch",
    "bundle",
    "checkout",
    "cherry",
    "cherry-pick",
    "clean",
    "clone",
    "commit",
    "config",
    "describe",
    "diff",
    "difftool",
    "fetch",
    "format-patch",
    "gc",
    "grep",
    "help",
    "init",
    "log",
    "merge",
    "mergetool",
    "mv",
    "notes",
    "pull",
    "push",
    "rebase",
    "reflog",
    "remote",
    "reset",
    "restore",
    "revert",
    "rm",
    "shortlog",
    "show",
    "stash",
    "status",
    "submodule",
    "switch",
    "tag",
    "worktree",
];

pub fn complete(words: &[String], position: usize, config: &Config) -> Vec<String> {
    if words.is_empty() {
        return command_completions("");
    }

    let args = words.get(1..).unwrap_or(&[]);
    let relative_position = position.saturating_sub(1);
    let current = args
        .get(relative_position)
        .map(String::as_str)
        .unwrap_or("");

    if relative_position == 0 {
        return command_completions(current);
    }

    let Some(raw_command) = args
        .first()
        .map(String::as_str)
        .filter(|word| !word.is_empty())
    else {
        return command_completions(current);
    };

    let command = expand_alias_command(raw_command).unwrap_or_else(|| raw_command.to_string());
    let rest = args.get(1..).unwrap_or(&[]);

    if let Some(values) = option_value_completions(&command, current) {
        return values;
    }
    if current.starts_with("--") {
        return long_option_completions(&command, current);
    }
    if current.starts_with('-') && current != "-" {
        return short_option_completions(&command, current);
    }

    match command.as_str() {
        "bisect" => subcommand_completions(
            &[
                "start",
                "bad",
                "good",
                "skip",
                "reset",
                "visualize",
                "replay",
                "log",
                "run",
            ],
            current,
        ),
        "flow" => complete_flow(rest, relative_position.saturating_sub(1), current),
        "remote" => complete_remote(rest, relative_position.saturating_sub(1), current),
        "stash" => complete_stash(rest, relative_position.saturating_sub(1), current),
        "push" | "pull" | "fetch" => {
            complete_transport(&command, rest, relative_position - 1, current)
        }
        "add" => complete_files(config, current, |changes| {
            chain_paths([
                &changes.working.unmerged,
                &changes.working.modified,
                &changes.working.added,
            ])
        }),
        "checkout" => complete_checkout(config, rest, current),
        "restore" => complete_restore(config, rest, current),
        "reset" => complete_reset(config, rest, current),
        "rm" => complete_files(config, current, |changes| changes.working.deleted.clone()),
        "diff" | "difftool" => complete_diff(config, rest, current),
        "merge" | "mergetool" => complete_merge(config, rest, current),
        "switch" => complete_refs(current, true, true, false),
        "worktree" => complete_worktree(rest, relative_position - 1, current),
        "cherry" | "cherry-pick" | "log" | "rebase" | "reflog" | "revert" | "show" => {
            complete_refs(current, true, true, false)
        }
        "help" => command_completions(current),
        _ => Vec::new(),
    }
}

fn command_completions(filter: &str) -> Vec<String> {
    let mut commands = COMMANDS
        .iter()
        .copied()
        .filter(|command| command.starts_with(filter))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    commands.extend(git::git_alias_names(filter));
    commands.sort();
    commands.dedup();
    commands
}

fn expand_alias_command(command: &str) -> Option<String> {
    let alias = git::git_alias(command)?;
    if alias.starts_with('!') {
        if alias.contains("vsts code pr") {
            return Some("vsts.pr".to_string());
        }
        return None;
    }

    alias.split_whitespace().next().map(ToOwned::to_owned)
}

fn complete_remote(rest: &[String], relative_position: usize, current: &str) -> Vec<String> {
    let subcommands = [
        "add",
        "rename",
        "remove",
        "set-head",
        "set-branches",
        "get-url",
        "set-url",
        "show",
        "prune",
        "update",
    ];
    if relative_position == 0 {
        return subcommand_completions(&subcommands, current);
    }

    let operation = rest.first().map(String::as_str).unwrap_or("");
    if matches!(
        operation,
        "rename" | "remove" | "rm" | "set-head" | "set-branches" | "set-url" | "show" | "prune"
    ) {
        git::git_remotes(current)
    } else {
        Vec::new()
    }
}

fn complete_stash(rest: &[String], relative_position: usize, current: &str) -> Vec<String> {
    let subcommands = [
        "push", "save", "list", "show", "apply", "clear", "drop", "pop", "create", "branch",
    ];
    if relative_position == 0 {
        return subcommand_completions(&subcommands, current);
    }

    let operation = rest.first().map(String::as_str).unwrap_or("");
    if matches!(operation, "show" | "apply" | "drop" | "pop" | "branch") {
        git::git_stashes(current)
    } else {
        Vec::new()
    }
}

fn complete_flow(rest: &[String], relative_position: usize, current: &str) -> Vec<String> {
    if relative_position == 0 {
        return subcommand_completions(
            &[
                "init", "feature", "bugfix", "release", "hotfix", "support", "help", "version",
            ],
            current,
        );
    }

    if relative_position == 1 {
        return match rest.first().map(String::as_str).unwrap_or("") {
            "feature" | "bugfix" => subcommand_completions(
                &[
                    "list", "start", "finish", "publish", "track", "diff", "rebase", "checkout",
                    "pull", "help", "delete",
                ],
                current,
            ),
            "release" | "hotfix" => subcommand_completions(
                &[
                    "list", "start", "finish", "track", "publish", "help", "delete",
                ],
                current,
            ),
            "support" => subcommand_completions(&["list", "start", "help"], current),
            _ => Vec::new(),
        };
    }

    complete_refs(current, false, false, false)
}

fn complete_transport(
    command: &str,
    rest: &[String],
    relative_position: usize,
    current: &str,
) -> Vec<String> {
    let non_options = non_option_words(rest);
    let current_absolute = relative_position;
    let current_non_option = non_options
        .iter()
        .position(|(index, _)| *index == current_absolute);

    if current_non_option == Some(0) || (current_non_option.is_none() && non_options.is_empty()) {
        return git::git_remotes(current);
    }

    let remote = non_options
        .first()
        .map(|(_, value)| value.as_str())
        .unwrap_or("");
    if remote.is_empty() {
        return Vec::new();
    }

    if command == "fetch" && current_non_option == Some(0) {
        return git::git_remotes(current);
    }

    complete_refspec(remote, current)
}

fn complete_refspec(remote: &str, current: &str) -> Vec<String> {
    let (force, rest) = current
        .strip_prefix('+')
        .map_or(("", current), |rest| ("+", rest));
    if let Some((left, filter)) = rest.split_once(':') {
        let ref_prefix = format!("{left}:");
        return git::git_remote_branches(remote, &ref_prefix, filter, force);
    }

    let mut refs = complete_refs(rest, true, true, false)
        .into_iter()
        .map(|value| format!("{force}{value}"))
        .collect::<Vec<_>>();
    refs.sort();
    refs
}

fn complete_checkout(config: &Config, rest: &[String], current: &str) -> Vec<String> {
    if rest.iter().any(|word| word == "--") {
        return complete_files(config, current, |changes| {
            chain_paths([
                &changes.working.unmerged,
                &changes.working.modified,
                &changes.working.deleted,
            ])
        });
    }

    let mut refs = complete_refs(current, true, true, true);
    refs.sort();
    refs.dedup();
    refs
}

fn complete_restore(config: &Config, rest: &[String], current: &str) -> Vec<String> {
    if let Some(filter) = current.strip_prefix("--source=") {
        return complete_refs(filter, true, true, false)
            .into_iter()
            .map(|value| format!("--source={value}"))
            .collect();
    }

    if rest
        .iter()
        .rev()
        .nth(1)
        .map(|word| word == "-s")
        .unwrap_or(false)
    {
        return complete_refs(current, true, true, false);
    }

    let staged = rest.iter().any(|word| word == "--staged" || word == "-S");
    complete_files(config, current, |changes| {
        if staged {
            chain_paths([
                &changes.index.added,
                &changes.index.modified,
                &changes.index.deleted,
            ])
        } else {
            chain_paths([
                &changes.working.unmerged,
                &changes.working.modified,
                &changes.working.deleted,
            ])
        }
    })
}

fn complete_reset(config: &Config, rest: &[String], current: &str) -> Vec<String> {
    if rest.iter().any(|word| word == "HEAD") {
        complete_files(config, current, |changes| changes.index.paths())
    } else {
        complete_refs(current, true, true, false)
    }
}

fn complete_diff(config: &Config, rest: &[String], current: &str) -> Vec<String> {
    let staged = rest
        .iter()
        .any(|word| word == "--cached" || word == "--staged");
    complete_files(config, current, |changes| {
        if staged {
            changes.index.modified.clone()
        } else {
            chain_paths([
                &changes.working.unmerged,
                &changes.working.modified,
                &changes.index.modified,
            ])
        }
    })
}

fn complete_merge(config: &Config, rest: &[String], current: &str) -> Vec<String> {
    if rest.iter().any(|word| word == "--") {
        complete_files(config, current, |changes| changes.working.unmerged.clone())
    } else {
        complete_refs(current, true, true, false)
    }
}

fn complete_worktree(rest: &[String], relative_position: usize, current: &str) -> Vec<String> {
    if relative_position == 0 {
        return subcommand_completions(
            &["add", "list", "lock", "move", "prune", "remove", "unlock"],
            current,
        );
    }
    if rest.first().map(String::as_str) == Some("add") && relative_position >= 2 {
        return complete_refs(current, false, true, false);
    }
    Vec::new()
}

fn complete_refs(
    filter: &str,
    include_head_refs: bool,
    include_tags: bool,
    include_unique_remotes: bool,
) -> Vec<String> {
    let mut refs = git::git_branches(filter, include_head_refs, "");
    if include_unique_remotes {
        refs.extend(git::git_remote_unique_branches(filter));
    }
    if include_tags {
        refs.extend(git::git_tags(filter, ""));
    }
    refs.sort();
    refs.dedup();
    refs
}

fn complete_files<F>(config: &Config, filter: &str, selector: F) -> Vec<String>
where
    F: FnOnce(&CompletionStatus) -> Vec<String>,
{
    let mut status_config = config.clone();
    status_config.enable_file_status = true;
    status_config.enable_stash_status = false;

    let Some(status) = git::collect(&status_config).ok().flatten() else {
        return Vec::new();
    };

    let completion_status = CompletionStatus {
        index: status.index,
        working: status.working,
    };
    selector(&completion_status)
        .into_iter()
        .filter(|path| path.starts_with(filter))
        .collect()
}

struct CompletionStatus {
    index: ChangeSet,
    working: ChangeSet,
}

fn chain_paths<const N: usize>(groups: [&Vec<String>; N]) -> Vec<String> {
    let mut paths = Vec::new();
    for group in groups {
        for path in group {
            if !paths.contains(path) {
                paths.push(path.clone());
            }
        }
    }
    paths
}

fn non_option_words(words: &[String]) -> Vec<(usize, String)> {
    let mut result = Vec::new();
    let mut skip_next = false;
    for (index, word) in words.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if option_takes_value(word) {
            skip_next = !word.contains('=');
            continue;
        }

        if word.starts_with('-') {
            continue;
        }

        result.push((index, word.clone()));
    }
    result
}

fn option_takes_value(word: &str) -> bool {
    matches!(
        word,
        "--repo" | "--receive-pack" | "--exec" | "--recurse-submodules" | "--upload-pack"
    ) || word.starts_with("--repo=")
        || word.starts_with("--receive-pack=")
        || word.starts_with("--exec=")
        || word.starts_with("--recurse-submodules=")
        || word.starts_with("--upload-pack=")
}

fn subcommand_completions(candidates: &[&str], filter: &str) -> Vec<String> {
    candidates
        .iter()
        .copied()
        .filter(|candidate| candidate.starts_with(filter))
        .map(ToOwned::to_owned)
        .collect()
}

fn long_option_completions(command: &str, current: &str) -> Vec<String> {
    option_words(long_options(command))
        .into_iter()
        .map(|option| format!("--{option}"))
        .filter(|option| option.starts_with(current))
        .collect()
}

fn short_option_completions(command: &str, current: &str) -> Vec<String> {
    option_words(short_options(command))
        .into_iter()
        .map(|option| format!("-{option}"))
        .filter(|option| option.starts_with(current))
        .collect()
}

fn option_value_completions(command: &str, current: &str) -> Option<Vec<String>> {
    let body = current.strip_prefix("--")?;
    let (param, filter) = body.split_once('=')?;
    let values = param_values(command, param)?;
    Some(
        option_words(values)
            .into_iter()
            .filter(|value| value.starts_with(filter))
            .map(|value| format!("--{param}={value}"))
            .collect(),
    )
}

fn option_words(options: &'static str) -> Vec<&'static str> {
    options
        .split_whitespace()
        .filter(|value| !value.is_empty())
        .collect()
}

fn long_options(command: &str) -> &'static str {
    match command {
        "add" => {
            "dry-run verbose force interactive patch edit update all intent-to-add refresh ignore-errors renormalize"
        }
        "branch" => {
            "color no-color list abbrev= no-abbrev column no-column merged no-merged contains set-upstream track no-track set-upstream-to= unset-upstream edit-description delete create-reflog force move all verbose quiet"
        }
        "checkout" => {
            "quiet force ours theirs track no-track detach orphan ignore-skip-worktree-bits merge conflict= patch"
        }
        "cherry-pick" => {
            "edit mainline no-commit signoff gpg-sign ff allow-empty allow-empty-message strategy= strategy-option= continue quit abort"
        }
        "clean" => "force interactive dry-run quiet exclude=",
        "clone" => {
            "local no-hardlinks shared reference quiet verbose progress no-checkout bare mirror origin branch upload-pack template= config depth single-branch no-single-branch recursive recurse-submodules separate-git-dir="
        }
        "commit" => {
            "all patch reuse-message reedit-message fixup squash reset-author short branch porcelain long null file author date message template signoff no-verify allow-empty allow-empty-message cleanup= edit no-edit amend include only untracked-files verbose quiet dry-run status no-status gpg-sign no-gpg-sign"
        }
        "config" => {
            "replace-all add get get-all get-regexp get-urlmatch global system local file blob remove-section rename-section unset unset-all list bool int bool-or-int path null edit includes no-includes"
        }
        "describe" => {
            "dirty all tags contains abbrev candidates= exact-match debug long match always first-parent"
        }
        "diff" => {
            "cached staged patch no-patch unified= raw minimal patience histogram diff-algorithm= stat numstat shortstat dirstat summary name-only name-status submodule color no-color word-diff word-diff-regex color-words no-renames check full-index binary break-rewrites find-renames find-copies ignore-space-at-eol ignore-space-change ignore-all-space ignore-blank-lines exit-code quiet ext-diff no-ext-diff textconv no-textconv ignore-submodules"
        }
        "difftool" => "dir-diff no-prompt prompt tool= tool-help no-symlinks symlinks extcmd= gui",
        "fetch" => {
            "all append depth= unshallow update-shallow dry-run force keep multiple prune no-tags tags recurse-submodules= no-recurse-submodules upload-pack quiet verbose progress"
        }
        "grep" => {
            "cached no-index untracked no-exclude-standard text textconv no-textconv ignore-case max-depth word-regexp invert-match full-name extended-regexp basic-regexp perl-regexp fixed-strings line-number files-with-matches null count color no-color break heading show-function context after-context before-context function-context and or not all-match quiet"
        }
        "help" => "all guides info man web",
        "init" => "quiet bare template= separate-git-dir= shared=",
        "log" => {
            "follow no-decorate decorate source use-mailmap full-diff max-count skip since after until before author committer grep all-match regexp-ignore-case basic-regexp extended-regexp fixed-strings perl-regexp remove-empty merges no-merges min-parents max-parents first-parent all branches tags remotes glob= exclude= bisect stdin cherry-mark cherry-pick left-only right-only walk-reflogs merge boundary simplify-by-decoration full-history ancestry-path date-order author-date-order topo-order reverse no-walk= do-walk pretty= format= abbrev-commit no-abbrev-commit oneline encoding= notes no-notes show-signature relative-date date= graph"
        }
        "merge" => {
            "commit no-commit edit no-edit ff no-ff ff-only log no-log stat no-stat squash no-squash strategy strategy-option verify-signatures no-verify-signatures summary no-summary quiet verbose progress abort allow-unrelated-histories"
        }
        "mergetool" => "tool= tool-help no-prompt prompt",
        "pull" => {
            "quiet verbose recurse-submodules= no-recurse-submodules= commit no-commit edit no-edit ff no-ff ff-only log no-log stat no-stat squash no-squash strategy= strategy-option= verify-signatures no-verify-signatures summary no-summary rebase= no-rebase all append depth= unshallow force keep no-tags upload-pack progress"
        }
        "push" => {
            "all prune mirror dry-run porcelain delete tags follow-tags receive-pack= exec= force-with-lease no-force-with-lease force repo= set-upstream thin no-thin quiet verbose progress recurse-submodules= verify no-verify"
        }
        "rebase" => {
            "onto continue abort keep-empty skip edit-todo merge strategy= strategy-option= gpg-sign quiet verbose stat no-stat no-verify verify force-rebase fork-point no-fork-point ignore-whitespace whitespace= committer-date-is-author-date ignore-date interactive exec root autosquash no-autosquash autostash no-autostash no-ff"
        }
        "remote" => "verbose",
        "reset" => "patch quiet soft mixed hard merge keep",
        "restore" => {
            "source= patch worktree staged quiet progress no-progress ours theirs merge conflict= ignore-unmerged ignore-skip-worktree-bits overlay no-overlay"
        }
        "revert" => {
            "edit mainline no-edit no-commit gpg-sign signoff strategy= strategy-option= continue quit abort"
        }
        "rm" => "force dry-run cached ignore-unmatch quiet",
        "show" => {
            "pretty= format= abbrev-commit no-abbrev-commit oneline encoding= expand-tabs no-expand-tabs notes no-notes show-signature name-only name-status stat shortstat numstat"
        }
        "stash" => "patch no-keep-index keep-index include-untracked all quiet index",
        "status" => {
            "short branch porcelain long untracked-files ignore-submodules ignored column no-column"
        }
        "switch" => {
            "create force-create detach guess no-guess force discard-changes merge conflict= quiet no-progress track no-track orphan ignore-other-worktrees recurse-submodules no-recurse-submodules"
        }
        "tag" => {
            "annotate sign local-user force delete verify list sort column no-column contains points-at message file cleanup"
        }
        _ => "",
    }
}

fn short_options(command: &str) -> &'static str {
    match command {
        "add" => "n v f i p e u A N",
        "blame" => "b L l t S p M C h c f n s e w",
        "branch" => "d D l f m M r a v q t u",
        "checkout" => "q f b B t l m p",
        "cherry" => "v",
        "cherry-pick" => "e x r m n s S X",
        "clean" => "d f i n q e x X",
        "clone" => "l s q v n o b u c",
        "commit" => "a p C c z F m t s n e i o u v q S",
        "config" => "f l z e",
        "diff" => "p u s U z B M C D l S G O R a b w W",
        "difftool" => "d y t x g",
        "fetch" => "a f k p n t u q v",
        "grep" => "a i I w v h H E G P F n l L O z c p C A B W f e q",
        "help" => "a g i m w",
        "init" => "q",
        "log" => "L n i E F g c m r t",
        "merge" => "e n s X q v S m",
        "mergetool" => "t y",
        "mv" => "f k n v",
        "notes" => "f m F C c n s q v",
        "pull" => "q v e n s X r a f k u",
        "push" => "n f u q v",
        "rebase" => "m s X S q v n C f i p x",
        "remote" => "v",
        "reset" => "q p",
        "restore" => "s p W S q m",
        "revert" => "e m n S s X",
        "rm" => "f n r q",
        "shortlog" => "n s e w",
        "stash" => "p k u a q",
        "status" => "s b u z",
        "submodule" => "q b f n N",
        "switch" => "c C d f m q t",
        "tag" => "a s u f d v n l m F",
        "whatchanged" => "p",
        _ => "",
    }
}

fn param_values(command: &str, param: &str) -> Option<&'static str> {
    match (command, param) {
        ("branch", "color") | ("diff", "color") => Some("always never auto"),
        ("branch", "abbrev") | ("diff", "abbrev") => Some("7 8 9 10"),
        ("checkout", "conflict") | ("restore", "conflict") | ("switch", "conflict") => {
            Some("merge diff3")
        }
        ("commit", "cleanup") => Some("strip whitespace verbatim scissors default"),
        ("diff", "unified") => Some("0 1 2 3 4 5"),
        ("diff", "diff-algorithm") => Some("default patience minimal histogram myers"),
        ("diff", "word-diff") => Some("color plain porcelain none"),
        ("diff", "ignore-submodules") | ("status", "ignore-submodules") => {
            Some("none untracked dirty all")
        }
        ("fetch", "recurse-submodules")
        | ("fetch", "recurse-submodules-default")
        | ("pull", "recurse-submodules")
        | ("pull", "no-recurse-submodules") => Some("yes on-demand no"),
        ("init", "shared") => Some("false true umask group all world everybody"),
        ("log", "pretty") | ("log", "format") | ("show", "pretty") | ("show", "format") => {
            Some("oneline short medium full fuller email raw")
        }
        ("log", "date") => Some("relative local default iso rfc short raw"),
        ("merge", "strategy")
        | ("pull", "strategy")
        | ("rebase", "strategy")
        | ("revert", "strategy")
        | ("cherry-pick", "strategy") => Some("resolve recursive octopus ours subtree"),
        ("push", "recurse-submodules") => Some("check on-demand"),
        ("pull", "rebase") => Some("false true preserve"),
        ("status", "untracked-files") => Some("no normal all"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completes_commands() {
        let completions = complete(&["git".into(), "ch".into()], 1, &Config::default());
        assert!(completions.contains(&"checkout".to_string()));
        assert!(completions.contains(&"cherry-pick".to_string()));
    }

    #[test]
    fn completes_long_options() {
        let completions = complete(
            &["git".into(), "push".into(), "--fo".into()],
            2,
            &Config::default(),
        );
        assert_eq!(
            completions,
            [
                "--follow-tags".to_string(),
                "--force-with-lease".to_string(),
                "--force".to_string()
            ]
        );
    }

    #[test]
    fn completes_option_values() {
        let completions = complete(
            &["git".into(), "status".into(), "--untracked-files=n".into()],
            2,
            &Config::default(),
        );
        assert_eq!(
            completions,
            [
                "--untracked-files=no".to_string(),
                "--untracked-files=normal".to_string()
            ]
        );
    }
}
