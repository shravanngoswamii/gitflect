---
layout: ../../layouts/DocsLayout.astro
title: Documentation
description: Install, configure, and understand gitflect.
---

`gitflect` is a small native prompt helper that shows Git state without taking over your shell.

It renders the branch, upstream divergence, staged and working tree changes, untracked files, stash count, and in-progress operations as a compact segment beside the prompt you already use.

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/install.sh | sh
```

The install script downloads the binary, puts it on your `PATH`, and wires up the shell integration automatically. Open a new terminal and you're done.

## Quick check

```sh
gitflect status --no-color
```

Inside a Git repository this prints a segment such as:

```text
(main ≡ +1 ~2 -0 !)
```

## Next steps

- [Themes](/docs/use/themes/) — switch the visual style or build a custom one
- [Settings](/docs/use/settings/) — browse and change all options interactively
