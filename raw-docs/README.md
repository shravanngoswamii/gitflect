# gitflect docs

`gitflect` is a small native prompt helper that shows Git state without taking
over your shell.

It keeps repository context close to the command line. It renders the branch,
upstream divergence, staged changes, working tree changes, untracked files,
stash count, and in-progress operations as a compact segment beside the prompt
you already use.

`gitflect` is inspired by posh-git, but built as a native Unix-style binary for
Linux and macOS terminals.

## First run

```sh
curl -fsSL https://raw.githubusercontent.com/shravangoswami/gitflect/main/install.sh | sh
```

Bash:

```sh
eval "$(gitflect init bash)"
```

Zsh:

```sh
eval "$(gitflect init zsh)"
```

Use `gitflect status --no-color` to inspect the rendered Git segment directly.

## Contents

- [Install](start/install.md)
- [Shell setup](start/shell-setup.md)
- [Update](start/update.md)
- [Prompt segments](use/prompt-segments.md)
- [Configuration](use/configuration.md)
- [Completions](use/completions.md)
- [Commands](reference/commands.md)
- [Release artifacts](reference/release-artifacts.md)
