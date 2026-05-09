---
layout: ../../../layouts/DocsLayout.astro
title: Configuration
description: Configure rendering, status behavior, and symbols.
---

Configuration is a small key-value file with optional environment overrides. All settings have defaults and work without a config file.

## View active config

To see every setting that is currently in effect (file + env overrides combined):

```sh
gitflect config
```

## Create the config file

```sh
gitflect config init
```

Creates `~/.config/gitflect/config` (or the path in `$XDG_CONFIG_HOME`) with all defaults written out. Edit it in any text editor.

To see where the file lives:

```sh
gitflect config path
```

To print the defaults without creating the file:

```sh
gitflect config default
```

## Lookup order

1. `$GITFLECT_CONFIG` (if set)
2. `$XDG_CONFIG_HOME/gitflect/config`
3. `~/.config/gitflect/config`

Environment variables are applied after the file, so they always win.

## Settings reference

| Key | Default | Description |
| :-- | :-- | :-- |
| `theme` | `posh` | Symbol set: `posh`, `plain`, `nerd` |
| `color` | `auto` | Color output: `auto`, `always`, `never` |
| `enable_prompt_status` | `true` | Show the Git segment in the prompt |
| `enable_file_status` | `true` | Show staged and working tree counts |
| `enable_stash_status` | `false` | Show stash count |
| `untracked_files` | `normal` | Untracked file detection: `no`, `normal`, `all` |
| `show_zero_counts` | `true` | Keep zero counts visible |
| `status_first` | `false` | Place status before path in full-prompt mode |
| `abbreviate_home` | `true` | Shorten home directory to `~` |
| `abbreviate_git_dir` | `false` | Shorten bare repo path |
| `branch_display` | `full` | Upstream display: `full`, `compact`, `minimal` |
| `branch_name_limit` | `0` | Truncate branch names longer than N (0 = off) |
| `prompt_suffix` | `> ` | Suffix appended after the prompt |
| `prompt_prefix` | _(empty)_ | Prefix before the path; auto-set to `[user@host]:` over SSH |
| `path_status_separator` | ` ` | Separator between path and Git segment |
| `show_exit_status` | `false` | Append last exit code when non-zero |

## Environment overrides

Every key has a `GITFLECT_` env var equivalent:

| Variable | Config key |
| :-- | :-- |
| `GITFLECT_THEME` | `theme` |
| `GITFLECT_COLOR` | `color` |
| `GITFLECT_ENABLE_STASH` | `enable_stash_status` |
| `GITFLECT_UNTRACKED_FILES` | `untracked_files` |
| `GITFLECT_SHOW_ZERO_COUNTS` | `show_zero_counts` |
| `GITFLECT_STATUS_FIRST` | `status_first` |
| `GITFLECT_BRANCH_NAME_LIMIT` | `branch_name_limit` |
| `GITFLECT_DISABLED_REPOSITORIES` | `disabled_repositories` |

To enable completions for Git aliases, set:

```sh
export GITFLECT_GIT_COMMANDS="git g"
```

## Symbols

Override any symbol with ASCII, Nerd Font glyphs, or any text:

```ini
symbol_added=+
symbol_modified=~
symbol_removed=-
symbol_conflicted=!
symbol_ahead=↑
symbol_behind=↓
symbol_identical=≡
symbol_diverged=↕
symbol_gone=gone
```

Or via env with `GITFLECT_SYMBOL_` prefix:

```sh
export GITFLECT_SYMBOL_AHEAD="▲"
export GITFLECT_SYMBOL_BEHIND="▼"
```

## Large repositories

Disable file status for specific worktrees (branch and operation state still show):

```ini
disabled_repositories=/work/huge-repo:/work/generated-repo
```
