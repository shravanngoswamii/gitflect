---
layout: ../../../layouts/DocsLayout.astro
title: Commands
description: Command reference for gitflect.
---

## `gitflect prompt`

Renders a prompt string.

```sh
gitflect prompt [--shell bash|zsh|raw|plain] [--status-only] [--last-status N] [--no-color|--color]
```

Use `--status-only` for shell integrations that preserve the user's existing prompt and insert only the Git segment.

## `gitflect status`

Renders the Git status segment for the current directory.

```sh
gitflect status [--json] [--shell bash|zsh|raw|plain] [--no-color|--color]
```

Use `--json` for scripts or tests that need structured repository state.

```sh
gitflect status --json --no-color
```

## `gitflect init`

Prints shell integration code for the selected shell.

```sh
gitflect init bash
gitflect init zsh
```

<div class="callout note">The install script runs this automatically. Manual use is only needed for custom setups — see <a href="/docs/start/shell-setup/">Shell setup</a>.</div>

## `gitflect settings`

Open the interactive settings explorer. Browse and edit all configuration options with keyboard navigation. Changes are written to the config file when you press `s`.

```sh
gitflect settings
```

**Key bindings:**

| Key | Action |
| :-- | :-- |
| **↑ / ↓** | Navigate between settings |
| **← / →** | Cycle enum or boolean values one step |
| **Enter** on enum field | Show all options inline — ↑/↓ navigate, Space/Enter select, Esc cancel |
| **Enter** on bool field | Toggle the value |
| **Enter** on text/number field | Open inline editor |
| **s** | Save all pending changes |
| **q** or **Esc** | Quit (prompts if there are unsaved changes) |

## `gitflect theme`

Lists available themes or switches the active theme.

### `gitflect theme list`

Print all available themes with a short description.

```sh
gitflect theme list
```

### `gitflect theme set <name>`

Switch to a named theme. Writes `theme=<name>` to the config file.

```sh
gitflect theme set posh
gitflect theme set posh-rounded
gitflect theme set plain
gitflect theme set nerd
gitflect theme set emoji
gitflect theme set minimal
gitflect theme set retro
```

For `custom`, an interactive wizard starts in the terminal. It walks through all 12 status segment symbols plus the bracket style (`bracket_open`/`bracket_close`) one at a time and writes the chosen values to the config file. After confirming, you can optionally give the theme a name to save it as a reusable file.

```sh
gitflect theme set custom
```

**Wizard key bindings:**

| Key | Action |
| :-- | :-- |
| Type text | Edit the current symbol |
| **Enter** or **↓** | Confirm current symbol and advance |
| **↑** | Go back to the previous field |
| **Backspace** | Delete the last character |
| **q** or **Esc** | Cancel without saving |

### `gitflect theme save <name>`

Save the current custom symbol set as a named, shareable theme file. Stored at `~/.config/gitflect/themes/<name>.conf`.

```sh
gitflect theme save mytheme
```

### `gitflect theme load <name>`

Load a saved named theme as the active custom theme. Sets `theme=custom` and writes all symbol keys to the config file.

```sh
gitflect theme load mytheme
```

### `gitflect theme saved`

List all saved named themes.

```sh
gitflect theme saved
```

## `gitflect config`

Shows all active settings (file + environment overrides combined), with valid options shown inline.

```sh
gitflect config
```

### `gitflect config get <key>`

Print the current value of a single key.

```sh
gitflect config get theme
# posh  # posh, posh-rounded, plain, nerd, emoji, minimal, retro, custom
```

### `gitflect config set <key> <value>`

Write a setting to the config file from the command line.

```sh
gitflect config set theme plain
gitflect config set color never
gitflect config set enable_stash_status true
```

If the key is unknown or the value is invalid, an error is printed with the list of valid options.

### `gitflect config path`

Print the path to the config file.

```sh
gitflect config path
```

### `gitflect config init`

Create the config file from defaults if it does not already exist.

```sh
gitflect config init
```

### `gitflect config default`

Print the default config template without creating a file.

```sh
gitflect config default
```

## `gitflect help`

Prints the built-in command summary.

```sh
gitflect help
```
