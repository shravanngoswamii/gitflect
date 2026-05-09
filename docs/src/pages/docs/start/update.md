---
layout: ../../../layouts/DocsLayout.astro
title: Update
description: Update gitflect from GitHub releases.
---

The installer is idempotent. Running it again replaces the existing `gitflect` binary with the requested release.

## Update to latest

```sh
curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/install.sh | sh
```

## Update a custom install directory

Use the same `BIN_DIR` you used during installation.

```sh
BIN_DIR="$HOME/.local/bin" curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/install.sh | sh
```

## Roll back

Install a previous tag explicitly.

```sh
GITFLECT_VERSION=v0.1.0 curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/install.sh | sh
```

## Confirm the active binary

```sh
command -v gitflect
gitflect --version
```

If the version does not change, check that the install directory appears before any older copy of `gitflect` on `PATH`.
