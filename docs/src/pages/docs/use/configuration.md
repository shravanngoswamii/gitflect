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

## Interactive settings explorer

The fastest way to browse and change settings without opening any files:

```sh
gitflect settings
```

Use **↑/↓** to navigate, **←/→** or **Enter** to cycle enum and boolean values, **Enter** to edit text fields inline, and **s** to save all changes at once.

## Config file path

To see where the file lives:

```sh
gitflect config path
```

## Lookup order

1. `$GITFLECT_CONFIG` (if set)
2. `$XDG_CONFIG_HOME/gitflect/config`
3. `~/.config/gitflect/config`

Environment variables are applied after the file, so they always win.

## Settings reference

| Key | Default | Valid values |
| :-- | :-- | :-- |
| `theme` | `posh-rounded` | `posh-rounded`, `posh`, `plain`, `nerd`, `emoji`, `minimal`, `retro`, `custom` |
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

## Themes

gitflect ships with four built-in themes:

| Theme | Description |
| :-- | :-- |
| `posh` | Unicode symbols — the default, inspired by posh-git (`↑ ↓ ↕ ≡ ×`) |
| `posh-rounded` | Same as `posh` but wraps the status block with `( )` instead of `[ ]` |
| `plain` | ASCII text labels only (`ahead`, `behind`, `<>`, `=`, `gone`) |
| `nerd` | Nerd Font glyph icons — requires a Nerd Font patched terminal font |
| `emoji` | Single-width Unicode symbols (`⬆ ⬇ ⇅ ✔ ✘ ✚ ✎ ✖`) |
| `minimal` | Single ASCII char per segment (`^ v x = ~ + * -`) |
| `retro` | Bracket-style labels (`>> << >< [+] [~] [-] [!]`) |
| `custom` | Your own symbols — configure interactively with the wizard |

### Custom theme

Run the interactive wizard to define every symbol:

```sh
gitflect theme set custom
```

The wizard walks through all 12 status segments one at a time. Use **↑ / ↓** to navigate between fields, **Enter** to confirm a value, and **q** to cancel without saving. Pressing Enter on an empty field keeps the current value.

After you confirm all symbols the wizard shows a summary and asks for confirmation before writing to your config file. It then prompts for an optional name — enter one to save the theme as a shareable file.

#### Save and share themes

Save the current custom symbol set as a named theme:

```sh
gitflect theme save mytheme
```

Load it on any machine that has the file:

```sh
gitflect theme load mytheme
```

List saved themes:

```sh
gitflect theme saved
```

Named theme files live at `~/.config/gitflect/themes/<name>.conf` and contain only `symbol_*` keys, so they are easy to share or version-control.

You can also set individual symbols manually without the wizard:

```ini
# theme must be custom for symbol_* keys to take effect without override
theme=custom
symbol_ahead=▲
symbol_behind=▼
symbol_identical=✓
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
