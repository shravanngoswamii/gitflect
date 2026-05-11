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

The install script detects your shell, places the binary on `PATH`, and wires up the prompt hook. Open a new terminal and you're done.

**Via Cargo**

```sh
cargo install gitflect
```

Then add the init line to your shell profile and reload:

```sh
# Bash — add to ~/.bashrc
echo 'eval "$(gitflect init bash)"' >> ~/.bashrc && exec $SHELL

# Zsh — add to ~/.zshrc
echo 'eval "$(gitflect init zsh)"' >> ~/.zshrc && exec $SHELL
```

To remove:

```sh
cargo uninstall gitflect
# then remove the eval line from ~/.bashrc or ~/.zshrc
```

## Usage

```sh
gitflect status              # print current Git segment
gitflect theme list          # list themes, active one marked with *
gitflect theme set minimal   # switch theme
gitflect settings            # browse and edit all settings interactively
gitflect --version
```

Prompt segment example:

```
user@host ~/project (main ≡ +1 ~0 -0 | !2 ~1 -0 ?)
```

See the [docs](https://shravangoswami.com/gitflect/) for [themes](https://shravangoswami.com/gitflect/docs/use/themes/) and [settings](https://shravangoswami.com/gitflect/docs/use/settings/).

## Development

```sh
git config core.hooksPath .githooks   # enable pre-commit fmt check (once per clone)
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
```

Issues at [github.com/shravanngoswamii/gitflect/issues](https://github.com/shravanngoswamii/gitflect/issues).
