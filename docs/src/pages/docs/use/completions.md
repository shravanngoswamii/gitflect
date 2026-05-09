---
layout: ../../../layouts/DocsLayout.astro
title: Completions
description: Git-aware completions for Bash and Zsh.
---

`gitflect` provides completion candidates through the same binary used for prompt rendering. The shell integration wires the completion function automatically.

## Enable completions

Bash:

```sh
eval "$(gitflect init bash)"
```

Zsh:

```sh
eval "$(gitflect init zsh)"
```

## What is completed

The completion engine focuses on common Git workflows:

| Area | Examples |
| :-- | :-- |
| Commands | `checkout`, `commit`, `merge`, `rebase`, `status` |
| Options | Long and short options for common commands |
| Branches | Local and remote branch names |
| Remotes | Remote names for fetch, pull, push, and remote commands |
| Tags | Tag names for checkout, switch, describe, and log workflows |
| Stashes | Stash references for apply, pop, drop, branch, and show |
| Changed files | Context-aware paths for add, restore, reset, diff, merge, and rm |

## Aliases

Attach completions to additional command names with `GITFLECT_GIT_COMMANDS`.

```sh
export GITFLECT_GIT_COMMANDS="git g"
eval "$(gitflect init bash)"
```

## Direct completion command

Shell integrations call this command internally:

```sh
gitflect complete --shell bash --position 2 -- git checkout ma
```

It prints one candidate per line. This makes completion behavior testable without starting an interactive shell.
