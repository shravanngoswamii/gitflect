---
layout: ../../../layouts/DocsLayout.astro
title: Themes
description: Switch and customise the visual style of the gitflect prompt segment.
---

gitflect ships with eight built-in themes. Switch between them with one command — no config file editing needed.

## List themes

```sh
gitflect theme list
```

The active theme is marked with `*` in green.

## Switch theme

```sh
gitflect theme set posh-rounded
gitflect theme set posh
gitflect theme set plain
gitflect theme set nerd
gitflect theme set emoji
gitflect theme set minimal
gitflect theme set retro
```

| Theme | Style |
| :-- | :-- |
| `posh-rounded` | Unicode symbols with `( )` brackets — **default** |
| `posh` | Same symbols with `[ ]` brackets |
| `plain` | ASCII text labels (`ahead`, `behind`, `<>`, `=`, `gone`) |
| `nerd` | Nerd Font glyph icons — requires a patched terminal font |
| `emoji` | Single-width Unicode symbols (`⬆ ⬇ ⇅ ✔ ✘`) |
| `minimal` | One ASCII char per segment (`^ v x = ~ + * -`) |
| `retro` | Bracket-style labels (`>> << >< [+] [~] [-]`) |
| `custom` | Your own symbols — see below |

## Custom theme

Build your own by choosing a symbol for each status segment:

```sh
gitflect theme set custom
```

An interactive wizard opens in the terminal. Walk through each field one at a time — symbols, brackets, everything. Review the summary, then confirm to save.

### Save and share

Give a name to reuse or share a custom symbol set:

```sh
gitflect theme save mytheme   # save current symbols
gitflect theme load mytheme   # apply on any machine
gitflect theme saved          # list saved themes
```

Named theme files live at `~/.config/gitflect/themes/<name>.conf` and contain only `key=value` lines, so they are easy to copy or version-control.
