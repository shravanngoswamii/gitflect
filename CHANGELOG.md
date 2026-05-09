# Changelog

All notable changes to gitflect are documented here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

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
