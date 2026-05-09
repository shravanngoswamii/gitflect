---
layout: ../../layouts/DocsLayout.astro
title: Documentation
description: Install, configure, and understand gitflect.
---

`gitflect` is a small native prompt helper that shows Git state without taking
over your shell.

It keeps repository context close to the command line. It renders the branch,
upstream divergence, staged changes, working tree changes, untracked files,
stash count, and in-progress operations as a compact segment beside the prompt
you already use.

`gitflect` is inspired by posh-git, but built as a native Unix-style binary for
Linux and macOS terminals.

## First run

```sh
curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/install.sh | sh
```

Bash:

```sh
eval "$(gitflect init bash)"
```

Zsh:

```sh
eval "$(gitflect init zsh)"
```

Use `gitflect status --no-color` to inspect the rendered Git segment directly.

## Contents

- [Install](start/install/)
- [Shell setup](start/shell-setup/)
- [Update](start/update/)
- [Prompt segments](use/prompt-segments/)
- [Configuration](use/configuration/)
- [Completions](use/completions/)
- [Commands](reference/commands/)
- [Release artifacts](reference/release-artifacts/)
