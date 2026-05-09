---
layout: ../../../layouts/DocsLayout.astro
title: Install
description: Install gitflect from GitHub releases.
---

`gitflect` is installed as a single native binary. The default installer downloads the matching archive from GitHub Releases, places the binary in a user-writable bin directory, and can add the shell initialization block for Bash or Zsh.

## Install script

```sh
curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/install.sh | sh
```

The installer chooses a target archive based on your operating system and CPU:

| Platform | Release target |
| :-- | :-- |
| Linux x86_64 | `x86_64-unknown-linux-gnu` |
| Linux ARM64 | `aarch64-unknown-linux-gnu` |
| macOS Intel | `x86_64-apple-darwin` |
| macOS Apple silicon | `aarch64-apple-darwin` |

## Install location

By default, the binary is installed to a user-local bin directory. You can choose a different directory with `BIN_DIR`.

```sh
BIN_DIR="$HOME/.local/bin" sh -c "$(curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/install.sh)"
```

Make sure the selected directory is on `PATH`.

```sh
echo "$PATH" | tr ':' '\n'
```

## Version pinning

To install a specific release, pass `GITFLECT_VERSION`.

```sh
GITFLECT_VERSION=v0.1.0 curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/install.sh | sh
```

Use `latest` to follow the newest GitHub release.

```sh
GITFLECT_VERSION=latest curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/install.sh | sh
```

## Verify

```sh
gitflect --version
gitflect status --no-color
```

Outside a Git repository, `gitflect status` prints nothing. Inside a repository, it prints a compact segment such as:

```text
[main ≡ +1 ~2 -0 !]
```

<div class="callout note">Shell integration is added to your profile automatically by the install script. Manual setup is only needed if you placed the binary on <code>PATH</code> yourself — see <a href="/docs/start/shell-setup/">Shell setup</a>.</div>
