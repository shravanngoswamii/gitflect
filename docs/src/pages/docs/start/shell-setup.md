---
layout: ../../../layouts/DocsLayout.astro
title: Shell setup
description: Enable gitflect in Bash or Zsh.
---

<div class="callout note">If you installed gitflect with the install script, shell integration is already configured automatically. This page is only needed for manual installs.</div>

`gitflect init` prints shell code. Add the matching line to your shell startup file after your normal prompt has been configured.

## Bash

```sh
eval "$(gitflect init bash)"
```

Common startup files are `~/.bashrc` on Linux and `~/.bash_profile` or `~/.bashrc` on macOS, depending on how your terminal starts Bash.

## Zsh

```sh
eval "$(gitflect init zsh)"
```

The usual startup file is `~/.zshrc`.

## Preserving your prompt

The default integration keeps your existing prompt and inserts the Git segment before the prompt marker.

```text
seeker@fedora:~/work/app [main ≡ +1 ~2 -0 !]$
```

Set `GITFLECT_REPLACE_PROMPT=1` before loading the init script if you want `gitflect` to render the whole prompt.

```sh
export GITFLECT_REPLACE_PROMPT=1
eval "$(gitflect init bash)"
```

## Try it without changing profile files

Bash:

```sh
bash --noprofile --norc -i
eval "$(gitflect init bash)"
```

Zsh:

```sh
zsh -f
eval "$(gitflect init zsh)"
```

Close the temporary shell to return to your normal setup.
