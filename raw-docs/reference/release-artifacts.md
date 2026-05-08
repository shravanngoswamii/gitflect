---
title: Release artifacts
description: GitHub release files used by the installer.
---

The release workflow publishes compressed archives for each supported platform. Each archive contains the `gitflect` binary, README, and license.

## Archive names

| Target | Archive |
| :-- | :-- |
| Linux x86_64 | `gitflect-x86_64-unknown-linux-gnu.tar.gz` |
| Linux ARM64 | `gitflect-aarch64-unknown-linux-gnu.tar.gz` |
| macOS Intel | `gitflect-x86_64-apple-darwin.tar.gz` |
| macOS Apple silicon | `gitflect-aarch64-apple-darwin.tar.gz` |

Each archive also has a `.sha256` checksum file.

## Installer inputs

| Variable | Default | Purpose |
| :-- | :-- | :-- |
| `GITFLECT_REPO` | `shravangoswami/gitflect` | GitHub owner and repository |
| `GITFLECT_VERSION` | `latest` | Release tag or latest release |
| `GITFLECT_BASE_URL` | GitHub releases URL | Override download base URL |
| `BIN_DIR` | User-local bin directory | Install destination |

## Release checks

The release workflow validates that the pushed tag matches the Cargo package version. It then runs formatting, linting, tests, installer syntax checks, cross-platform builds, checksums, and draft release creation.

Tags should use semantic versions with a leading `v`.

```text
v0.1.0
```
