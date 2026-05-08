---
title: Configuration
description: Configure rendering, status behavior, and symbols.
---

Configuration is a small key-value file with optional environment overrides.

## Lookup order

`gitflect` reads the first available config path in this order:

1. `$GITFLECT_CONFIG`
2. `$XDG_CONFIG_HOME/gitflect/config`
3. `~/.config/gitflect/config`

Environment variables are applied after the file, so they always win.

## Generate defaults

```sh
gitflect config --print-default
```

Use that output as a starting point for your config file.

## Common options

```ini
theme=posh
color=auto
enable_prompt_status=true
enable_file_status=true
enable_stash_status=false
untracked_files=normal
show_zero_counts=true
status_first=false
branch_display=full
describe_style=contains
path_status_separator= 
```

## Environment overrides

| Variable | Config key | Purpose |
| :-- | :-- | :-- |
| `GITFLECT_CONFIG` | path only | Reads a specific config file |
| `GITFLECT_THEME` | `theme` | Selects the built-in theme |
| `GITFLECT_COLOR` | `color` | Uses `auto`, `always`, or `never` |
| `GITFLECT_ENABLE_STASH_STATUS` | `enable_stash_status` | Shows stash counts |
| `GITFLECT_UNTRACKED_FILES` | `untracked_files` | Uses Git's untracked file modes |
| `GITFLECT_SHOW_ZERO_COUNTS` | `show_zero_counts` | Keeps or hides zero counts |
| `GITFLECT_STATUS_FIRST` | `status_first` | Places status before the path in full prompt mode |
| `GITFLECT_BRANCH_NAME_LIMIT` | `branch_name_limit` | Truncates long branch names |
| `GITFLECT_DISABLED_REPOSITORIES` | `disabled_repositories` | Skips file status in selected worktrees |

## Symbols

Symbols can be replaced with ASCII, Nerd Font glyphs, or any text your terminal can render.

```ini
symbol_added=+
symbol_modified=~
symbol_removed=-
symbol_conflicted=!
symbol_ahead=↑
symbol_behind=↓
symbol_identical=≡
symbol_gone=gone
```

Equivalent environment overrides are available with the `GITFLECT_SYMBOL_` prefix.

```sh
export GITFLECT_SYMBOL_AHEAD="ahead "
export GITFLECT_SYMBOL_BEHIND="behind "
```

## Large repositories

For repositories where file status is expensive, disable file status for specific worktrees.

```ini
disabled_repositories=/work/huge-repo,/work/generated-repo
```

The branch and operation state remain available, while staged and working tree counts are skipped.
