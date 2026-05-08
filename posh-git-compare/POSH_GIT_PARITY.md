# posh-git Parity Notes

The sibling `../posh-git/` checkout was used as the behavioral reference. The main
contracts translated here are:

- `GitUtils.ps1`: discover `.git`, parse `git status --short --branch`, resolve
  branch and in-progress operation state.
- `GitPrompt.ps1`: render branch, upstream, index, working tree, local summary,
  stash count, delimiters, and prompt/path layout.
- `PoshGitTypes.ps1`: expose settings for symbols, colors, file-status behavior,
  stash behavior, path abbreviation, branch truncation, and ahead/behind display.
- `GitTabExpansion.ps1` and `GitParamTabExpansion.ps1`: complete common commands,
  aliases, refs, remotes, options, option values, stashes, and status-derived paths.

## Adaptations for Bash/Zsh

- Prompt rendering is done by a native binary instead of shell script.
- ANSI escape sequences are wrapped for prompt length accounting:
  Bash uses `\[...\]`; Zsh uses `%{...%}`.
- The shell hook preserves the previous command exit status while rebuilding the
  prompt on each prompt draw.
- Completion is implemented as a `complete` subcommand consumed by Bash/Zsh
  completion functions.
- Configuration uses a small key-value file and `GITFLECT_*` environment
  overrides instead of a live PowerShell settings object.

## Current Coverage

Implemented:

- branch and detached HEAD display
- upstream identical/ahead/behind/diverged/gone
- staged/working added, modified, deleted, conflicted counts
- untracked file indicator through working added files
- optional stash count
- merge/rebase/apply/cherry-pick/revert/bisect state suffixes
- path and status ordering
- home and repository path abbreviation
- Bash/Zsh prompt integration
- Git-aware completion for common commands, refs, remotes, options, option
  values, stashes, and changed files

Future parity work:

- async/cache-backed file status for very large repositories
- richer color customization beyond named prompt roles
- generated completion/manpage artifacts for release archives
- deeper native Git integration through a Rust Git library if needed
