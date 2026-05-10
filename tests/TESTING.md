# Local Testing Guide

Personal notes for testing gitflect changes locally without touching the live shell setup.

## 1. Build the dev binary

Always build from the project root before testing:

```sh
cargo build
# binary: ./target/debug/gitflect
```

For a release-equivalent build (strip + LTO):

```sh
cargo build --release
# binary: ./target/release/gitflect
```

## 2. Test one-off commands without touching real config

Point to a throwaway config file using the env var — no file is created until you explicitly write to it:

```sh
export GITFLECT_CONFIG=/tmp/gitflect-test.conf

# Preview status in the current repo
./target/debug/gitflect status

# Check active config (reads from /tmp/gitflect-test.conf or shows defaults)
./target/debug/gitflect config

# Test a theme without writing anything
GITFLECT_THEME=emoji ./target/debug/gitflect status
GITFLECT_THEME=retro ./target/debug/gitflect status
GITFLECT_THEME=minimal ./target/debug/gitflect status
GITFLECT_THEME=posh-rounded ./target/debug/gitflect status
```

You can stack env vars for one-liners without touching any file:

```sh
GITFLECT_THEME=retro GITFLECT_CONFIG=/dev/null ./target/debug/gitflect status
```

## 3. Sandbox shell (full prompt integration test)

Start an isolated bash session that has no bashrc and no shared history.
Exit with `exit` or Ctrl+D — your real shell is unaffected.

```sh
bash --norc --noprofile
```

Inside the sandbox:

```sh
# Point at a test config
export GITFLECT_CONFIG=/tmp/gitflect-sandbox.conf

# Add the dev binary to PATH (use absolute path — ~ doesn't expand in some contexts)
export PATH="$PWD/target/debug:$PATH"

# Wire up the prompt
eval "$(gitflect init bash)"

# Now your prompt shows git status. Navigate into any repo and test away.
cd /some/git/repo
```

To start fresh, just delete the test config and restart:

```sh
rm -f /tmp/gitflect-sandbox.conf
```

## 4. Test the custom theme wizard in a sandbox

```sh
# In the sandbox shell:
GITFLECT_CONFIG=/tmp/gitflect-wizard-test.conf gitflect theme set custom

# Inspect what was written
cat /tmp/gitflect-wizard-test.conf

# Preview the result
GITFLECT_CONFIG=/tmp/gitflect-wizard-test.conf gitflect status
```

## 5. Test a specific theme

```sh
# Fastest: env var override, no config file needed
GITFLECT_THEME=emoji     ./target/debug/gitflect status
GITFLECT_THEME=minimal   ./target/debug/gitflect status
GITFLECT_THEME=retro     ./target/debug/gitflect status
GITFLECT_THEME=posh-rounded ./target/debug/gitflect status

# Or via the theme command (writes to GITFLECT_CONFIG):
GITFLECT_CONFIG=/tmp/t.conf ./target/debug/gitflect theme set retro
GITFLECT_CONFIG=/tmp/t.conf ./target/debug/gitflect status
```

## 6. Run the test suite

```sh
cargo test
```

Tests live in `tests/cli_integration.rs`. They use a temporary git repo and run the binary directly — no shell setup needed.

To run a single test by name:

```sh
cargo test test_name_fragment
```

## 7. Iterate quickly after a change

```sh
# Rebuild + preview in one line (run from repo root, inside a git repo)
cargo build -q && GITFLECT_THEME=retro ./target/debug/gitflect status

# Or with the sandbox prompt: rebuild and re-init without restarting bash
cargo build -q && eval "$(gitflect init bash)"
```

## 8. Verify config round-trip

After making config changes, confirm the active config reflects them:

```sh
GITFLECT_CONFIG=/tmp/t.conf ./target/debug/gitflect config set theme posh-rounded
GITFLECT_CONFIG=/tmp/t.conf ./target/debug/gitflect config get theme
GITFLECT_CONFIG=/tmp/t.conf ./target/debug/gitflect config
```

## 9. Cleanup

Test config files:

```sh
rm -f /tmp/gitflect-*.conf /tmp/t.conf
```

The real config file is at `~/.config/gitflect/config` (or `$GITFLECT_CONFIG`).
As long as you always set `GITFLECT_CONFIG` in your sandbox, the real file is never touched.
