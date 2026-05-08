---
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

Prints shell integration code.

```sh
gitflect init bash
gitflect init zsh
```

The generated code installs the prompt hook and Git completion function for the selected shell.

## `gitflect complete`

Prints completion candidates.

```sh
gitflect complete --shell bash --position N -- WORDS...
```

Shells call this command from their completion functions. It can also be used directly while debugging completion behavior.

## `gitflect config`

Prints the default config.

```sh
gitflect config --print-default
```

## `gitflect help`

Prints the built-in command summary.

```sh
gitflect help
```
