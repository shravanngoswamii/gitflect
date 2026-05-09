# Changelog

All notable changes to gitflect are documented here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

## [0.1.1] - 2026-05-10

### Fixed

- Prompt segment duplicating on each command in VSCode's integrated terminal and any other terminal that wraps `PS1` with shell integration escape sequences after gitflect sets it. The original-prompt baseline is now preserved across re-runs of the shell init, and the per-command change detection that mis-identified terminal modifications as user changes has been removed.
- `source ~/.bashrc` no longer resets the stored baseline prompt to the already-modified value, so re-sourcing the profile is safe.

### Added

- Uninstall script (`uninstall.sh`) for clean removal of the binary and shell profile block.
- Improved install script output: step-by-step progress, explicit reload instruction (`source ~/.bashrc` / `source ~/.zshrc`), verify command, and removal instructions printed after install.
- `gitflect config` shows all active settings (file + env overrides).
- `gitflect config path` prints the config file path.
- `gitflect config init` creates the config file from defaults if it does not exist.
- `gitflect config default` prints the default config template (replaces `--print-default`).

## [0.1.0] - 2026-05-10

### Added

- Fast Git-aware prompt segment for Bash and Zsh
- Branch name and detached HEAD state (`HEAD:abc1234`)
- Upstream divergence tracking: ahead ↑, behind ↓, diverged ↕, in-sync ≡, gone ×
- Staged, working, and untracked file counts (`+1 ~2 -0 | !3 ~0 -0 ?1`)
- In-progress operation markers: MERGING, REBASE, REBASE-i, REBASE-m, AM, CHERRY-PICKING, REVERTING, BISECTING
- Stash count (opt-in via config)
- Three built-in themes: `posh`, `plain`, `nerd`
- Tab completion for 46 git subcommands with options and file path completions
- `gitflect init bash` and `gitflect init zsh` shell integration
- `gitflect config` for reading and listing configuration values
- Install script with automatic platform and shell detection
- Uninstall script for clean removal
- Documentation site
