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

Each enumerated setting shows its valid options in a comment on the line above.

## Read and write settings from the CLI

Get the current value of any key:

```sh
gitflect config get theme
# posh  # posh, plain, nerd
```

Set a value without opening the file:

```sh
gitflect config set theme plain
gitflect config set color never
gitflect config set enable_stash_status true
```

`set` writes to the config file and validates the value against the allowed options. If the key does not exist or the value is invalid, an error is printed with the valid choices.

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

| Key | Default | Valid values |
| :-- | :-- | :-- |
| `theme` | `posh` | `posh`, `plain`, `nerd` |
| `color` | `auto` | `auto`, `always`, `never` |
| `enable_prompt_status` | `true` | `true`, `false` |
| `enable_file_status` | `true` | `true`, `false` |
| `enable_stash_status` | `false` | `true`, `false` |
| `untracked_files` | `normal` | `no`, `normal`, `all` |
| `show_zero_counts` | `true` | `true`, `false` |
| `status_first` | `false` | `true`, `false` |
| `abbreviate_home` | `true` | `true`, `false` |
| `abbreviate_git_dir` | `false` | `true`, `false` |
| `branch_display` | `full` | `full`, `compact`, `minimal` |
| `branch_name_limit` | `0` | integer (0 = off) |
| `prompt_suffix` | `> ` | any string |
| `prompt_prefix` | _(empty)_ | any string; auto-set to `[user@host]:` over SSH |
| `path_status_separator` | ` ` | any string |
| `show_exit_status` | `false` | `true`, `false` |

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
