# gitflect

`gitflect` is a fast Git-aware prompt helper for Bash and Zsh on Linux and
macOS. It renders a compact repository segment, keeps your existing prompt
shape, and adds Git completions without turning your shell into a framework.

## Features

- branch and detached HEAD display
- ahead, behind, diverged, and upstream-gone indicators
- staged, unstaged, deleted, untracked, and conflicted counts
- merge, rebase, cherry-pick, revert, bisect, and apply states
- optional stash count
- Bash and Zsh prompt hooks
- Git-aware completions for common commands, refs, remotes, stashes, options, and changed files
- small key-value config file plus environment overrides

## Motivation

`gitflect` is inspired by posh-git: show the Git state where developers already
look, keep the signal concise, and make the prompt useful without making it
noisy. The implementation is native to Unix-like shells and built as a small
Rust binary.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/install.sh | sh
```

Re-run the installer to update to the latest GitHub release.

```bash
sh install.sh --version v0.1.0
sh install.sh --bin-dir "$HOME/bin"
sh install.sh --shell zsh --profile "$HOME/.zshrc"
sh install.sh --no-modify-profile
```

## Shell Setup

Bash:

```bash
eval "$(gitflect init bash)"
```

Zsh:

```zsh
eval "$(gitflect init zsh)"
```

By default, `gitflect` preserves your existing prompt and inserts the Git
segment before the normal prompt marker. Set `GITFLECT_REPLACE_PROMPT=1` before
loading the init script to use the full `gitflect` prompt renderer.

## Usage

```bash
gitflect status --no-color
gitflect prompt --no-color
gitflect config --print-default
```

Example status segment:

```text
[main ≡ +1 ~0 -0 | +0 ~2 -1 !]
```

## Configuration

Configuration is read from `$GITFLECT_CONFIG`, then
`$XDG_CONFIG_HOME/gitflect/config`, then `~/.config/gitflect/config`.
Environment variables override the file.

```bash
export GITFLECT_ENABLE_STASH=true
export GITFLECT_UNTRACKED_FILES=normal
export GITFLECT_SHOW_ZERO=true
export GITFLECT_STATUS_FIRST=false
export GITFLECT_THEME=posh
export GITFLECT_COLOR=auto
```

To enable completions for aliases:

```bash
export GITFLECT_GIT_COMMANDS="git g"
```

## Development

```bash
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
```

Release archives are built by GitHub Actions for:

- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
