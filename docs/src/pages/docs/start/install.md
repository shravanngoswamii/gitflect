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

## Via Cargo

If you have Rust installed, you can install directly from [crates.io](https://crates.io/crates/gitflect):

```sh
cargo install gitflect
```

The binary lands in `~/.cargo/bin`. Add the init line to your shell profile, then reload:

```sh
# Bash
echo 'eval "$(gitflect init bash)"' >> ~/.bashrc && exec $SHELL

# Zsh
echo 'eval "$(gitflect init zsh)"' >> ~/.zshrc && exec $SHELL
```

To update:

```sh
cargo install gitflect
```

To remove:

```sh
cargo uninstall gitflect
# then remove the eval line from ~/.bashrc or ~/.zshrc
```

## Update

### Install script

Rerun the install script — it overwrites the existing binary with the latest release:

```sh
curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/install.sh | sh
```

### Via Cargo

```sh
cargo install gitflect
```

`cargo install` replaces the existing binary in `~/.cargo/bin` with the latest version from crates.io. No shell profile changes are needed.

## Verify

```sh
gitflect --version
gitflect status --no-color
```

Outside a Git repository, `gitflect status` prints nothing. Inside a repository, it prints a compact segment such as:

```text
(main ≡ +1 ~2 -0 !)
```

After installation, reload your shell to activate it:

```sh
source ~/.bashrc   # or ~/.zshrc for Zsh
```

Or start a fresh session:

```sh
exec $SHELL
```

<div class="callout note">Shell integration is added to your profile automatically by the install script. Manual setup is only needed if you placed the binary on <code>PATH</code> yourself — see <a href="/docs/start/shell-setup/">Shell setup</a>.</div>
