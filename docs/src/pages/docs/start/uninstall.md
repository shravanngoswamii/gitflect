---
layout: ../../../layouts/DocsLayout.astro
title: Uninstall
description: Remove gitflect from your system.
---

## Uninstall script

The uninstall script removes the `gitflect` binary and the shell integration block from `~/.bashrc` and `~/.zshrc`.

```sh
curl -fsSL https://raw.githubusercontent.com/shravanngoswamii/gitflect/main/uninstall.sh | sh
```

After it runs, reload the shell to complete the removal:

```sh
exec $SHELL
```

Or simply open a new terminal.

## Manual removal

If you prefer to remove it by hand:

**1. Remove the binary**

```sh
rm -f ~/.local/bin/gitflect
```

If you installed to a custom directory, replace `~/.local/bin` with the path you used.

**2. Remove the shell integration block**

Open your shell profile (`~/.bashrc` or `~/.zshrc`) and delete the lines between and including the markers:

```sh
# >>> gitflect >>>
# ...
# <<< gitflect <<<
```

**3. Reload the shell**

```sh
exec $SHELL
```

## Verify removal

```sh
command -v gitflect || echo "gitflect removed"
```
