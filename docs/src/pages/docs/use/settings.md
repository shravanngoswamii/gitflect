---
layout: ../../../layouts/DocsLayout.astro
title: Settings
description: Browse and change all gitflect settings interactively.
---

`gitflect settings` opens an interactive settings explorer in the terminal. No config file editing needed.

```sh
gitflect settings
```

Use the keyboard to navigate and edit any setting. Changes are written to the config file when you save.

## Key bindings

| Key | Action |
| :-- | :-- |
| **↑ / ↓** | Move between settings |
| **← / →** | Cycle an enum or boolean value one step |
| **Enter** on enum field | Show all options — ↑/↓ to pick, Space/Enter to select |
| **Enter** on bool field | Toggle |
| **Enter** on text/number field | Open inline editor |
| **s** | Save all changes to the config file |
| **q** or **Esc** | Quit |

Pending (unsaved) changes are shown in yellow. The footer counts how many changes are waiting. Press `q` a second time to discard and exit without saving.

Selecting **custom** for the `theme` setting immediately launches the symbol wizard so you can configure your own prompt symbols without leaving the settings flow.

For the full list of available settings and their effects, see [Configuration](/docs/use/configuration/).
