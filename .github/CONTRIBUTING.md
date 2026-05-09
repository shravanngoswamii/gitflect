# Contributing to gitflect

Thanks for taking the time to contribute!

## Getting started

```sh
git clone https://github.com/shravanngoswamii/gitflect.git
cd gitflect
cargo build
cargo test
```

Enable the pre-commit hook so formatting is checked before each commit:

```sh
git config core.hooksPath .githooks
```

## Making changes

- **Bug fixes and small improvements** — open an issue first if the fix is non-obvious, then send a PR.
- **New features** — open an issue to discuss the idea before writing code.
- **Docs** — PRs for typos and clarifications are always welcome without an issue.

## Pull request checklist

- [ ] `cargo fmt --all` passes (the pre-commit hook checks this)
- [ ] `cargo test` passes
- [ ] Relevant docs in `docs/src/pages/` are updated
- [ ] CHANGELOG.md has an entry under `[Unreleased]`

## Commit style

One subject line, imperative mood, no period. Examples:

```
fix prompt duplication in VSCode integrated terminal
add config set/get subcommands
remove custom git tab completions
```

## Reporting bugs

Open an issue at <https://github.com/shravanngoswamii/gitflect/issues>.

For security issues, use the private advisory form at <https://github.com/shravanngoswamii/gitflect/security>.
