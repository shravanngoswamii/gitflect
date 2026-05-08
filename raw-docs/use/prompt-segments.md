---
title: Prompt segments
description: Understand the Git status segment rendered by gitflect.
---

The default prompt segment is compact and intentionally close to posh-git style.

```text
[main ≡ +1 ~2 -0 !]
```

## Branch and upstream

The first field is the branch or detached HEAD description.

| Example | Meaning |
| :-- | :-- |
| `main` | Current branch |
| `HEAD` | Detached HEAD when no better description is available |
| `v1.2.0` | Detached HEAD described by a tag when available |

Upstream state is shown next when the branch has tracking information.

| Symbol | Meaning |
| :-- | :-- |
| `≡` | Local branch and upstream are even |
| `↑2` | Local branch is ahead by two commits |
| `↓1` | Local branch is behind by one commit |
| `↓1 ↑2` | Local branch has diverged |
| `gone` | The configured upstream no longer exists |

## File counts

The segment separates staged changes from working tree changes with `|` when both are present.

```text
[main ≡ +1 ~0 -0 | +1 ~2 -1 !]
```

| Part | Meaning |
| :-- | :-- |
| First `+ ~ -` group | Staged additions, modifications, deletions |
| Second `+ ~ -` group | Working tree additions, modifications, deletions |
| `!` | Untracked files are present |
| `!3` | Merge conflicts are present |
| `$2` | Two stash entries are present when stash status is enabled |

Zero counts can be hidden or shown with configuration.

## In-progress operations

When Git is in the middle of an operation, `gitflect` annotates the branch name.

```text
[main|MERGING ≡ !1]
```

Supported operation indicators include merge, rebase, cherry-pick, revert, and bisect state.

## Status-only rendering

Use status-only mode when a shell integration needs just the Git segment.

```sh
gitflect prompt --status-only --no-color
```

Use full prompt mode when `gitflect` should render the path, status, and prompt suffix.

```sh
gitflect prompt --shell bash
```
