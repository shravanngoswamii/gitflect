# Changelog

All notable changes to gitflect are documented here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

## [0.4.0] - 2026-05-11

### Changed

- **`gitflect settings` custom theme** — selecting `custom` in the theme picker now launches the symbol wizard directly without leaving the settings flow.
- **Settings TUI** — enum picker is now a full-screen overlay with proper scrolling; terminal height is read dynamically instead of hardcoded, so all options are reachable on any terminal size.
- **Install script** — added `exec $SHELL` as a suggested alternative to `source ~/.bashrc` after a fresh install.

### Removed

- `gitflect config get`, `gitflect config set`, `gitflect config init`, `gitflect config default` — use `gitflect settings` for interactive editing and `gitflect config` / `gitflect config path` for inspection.

## [0.3.0] - 2026-05-11

### Changed

- **Default theme changed to `posh-rounded`** — prompts use `( )` brackets out of the box. Switch back with `gitflect theme set posh`.

### Added

- Docs: navigation restructured into Get started / Use / Advanced sections with dedicated Themes and Settings pages.

## [0.2.0] - 2026-05-11

### Added

- **Custom theme** (`theme=custom`) — a new theme variant that uses fully user-defined symbols. When set, no built-in symbols override user config; all 12 `symbol_*` keys are respected as-is.
- **`gitflect theme` command** — new top-level command for theme management:
  - `gitflect theme list` — show all available themes with descriptions.
  - `gitflect theme set <name>` — switch theme and write to config file.
  - `gitflect theme set custom` — launch the interactive wizard.
- **Interactive custom theme wizard** — a full-terminal form that walks through all 12 status segment symbols. Supports arrow-key navigation (↑/↓), inline editing with Backspace, Enter to confirm, and q/Esc to cancel. Shows a review summary before saving.
- **`posh-rounded` theme** — identical to `posh` but wraps the status block with `( )` instead of `[ ]`.
- **`emoji` theme** — single-width Unicode symbol set: `⬆ ⬇ ⇅ ✔ ✘ ✚ ✎ ✖ ⚡ ◉`.
- **`minimal` theme** — ultra-compact single ASCII char per segment: `^ v x = ~ + * - !`.
- **`retro` theme** — bracket-style labels: `>> << >< -- !! [+] [~] [-] [!]`.
- **`gitflect settings`** — interactive TUI settings explorer. Browse all 16 config options across five sections (Theme, Status, Branch, Prompt, Appearance) with ↑/↓ navigation. Cycle enum/bool values with ←/→ or Enter; edit text/number fields inline. Press `s` to save; press `q` twice to discard and quit.
- **Named themes** — save and share custom symbol sets as standalone `.conf` files:
  - `gitflect theme save <name>` — save current custom symbols to `~/.config/gitflect/themes/<name>.conf`.
  - `gitflect theme load <name>` — load a named theme file as the active custom theme.
  - `gitflect theme saved` — list all saved named themes.
  - The wizard's final step now optionally prompts for a name, saving the theme file in one step.
- **Go-back from summary in the wizard** — pressing ↑ or Esc on the review screen returns to the symbol editor instead of cancelling.
- **Bracket fields in wizard** — the custom theme wizard now includes `bracket_open`/`bracket_close` fields that configure `before_status`/`after_status` (e.g. `[`/`]` or `(`/`)`).
- **Named themes save/restore bracket style** — `gitflect theme save` and `gitflect theme load` now also persist and apply `before_status`/`after_status` alongside all symbol keys.
- **Active theme highlighted in `gitflect theme list`** — the currently active theme is marked with a `*` in green; other themes are shown dimmed.
- **Enum picker in `gitflect settings`** — pressing Enter on an enum field (e.g. `theme`, `untracked_files`, `branch_display`) opens an inline option picker; ↑/↓ navigate options, Space/Enter selects, Esc cancels.
- Docs: new "Themes" section in Configuration with wizard key bindings table and manual `symbol_*` override examples; new `gitflect theme` and `gitflect settings` sections in Commands reference.

## [0.1.2] - 2026-05-10

### Added

- `gitflect config set <key> <value>` — change a setting from the command line without editing the file.
- `gitflect config get <key>` — print the current value of a setting with valid options shown inline.
- `gitflect config` output now shows a `# key: option1, option2` comment above each enumerated setting so valid values are always visible.
- Community health files: `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, GitHub issue templates (bug report, feature request), and pull request template.
- Bug and security reporting links in `gitflect --help`.

### Removed

- Custom tab-completion override for Git subcommands. Bash and Zsh ship superior built-in Git completions; the gitflect override was disruptive and unnecessary.

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
