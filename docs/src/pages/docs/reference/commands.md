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

## `gitflect config`

Shows all active settings (file + environment overrides combined), with valid options shown inline.

```sh
gitflect config
```

### `gitflect config get <key>`

Print the current value of a single key.

```sh
gitflect config get theme
# posh  # posh, plain, nerd
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
