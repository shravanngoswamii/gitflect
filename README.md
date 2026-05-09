<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/public/logo-dark.svg">
    <img src="docs/public/logo-light.svg" alt="gitflect" width="240">
  </picture>
</p>

<p align="center">
  Fast Git context for Bash and Zsh. Branch, status, and counts right in the prompt.
</p>

<p align="center">
  <a href="https://shravangoswami.com/gitflect/">Docs</a>
  &nbsp;·&nbsp;
  <a href="CHANGELOG.md">Changelog</a>
  &nbsp;·&nbsp;
  <a href=".github/CONTRIBUTING.md">Contributing</a>
  &nbsp;·&nbsp;
  <a href="LICENSE">MIT</a>
</p>

```sh
curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/install.sh | sh
```

The install script detects your shell, places the binary on `PATH`, and wires up the prompt hook. No manual shell setup needed.

## Usage

```sh
gitflect status              # print current Git segment
gitflect config              # show active configuration
gitflect config set theme plain   # change a setting from the CLI
gitflect --version
```

Prompt segment example:

```
user@host ~/project [main ≡ +1 ~0 -0 | !2 ~1 -0 ?]
```

## Development

```sh
git config core.hooksPath .githooks   # enable pre-commit fmt check (once per clone)
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
```

Issues at [github.com/shravanngoswamii/gitflect/issues](https://github.com/shravanngoswamii/gitflect/issues).
