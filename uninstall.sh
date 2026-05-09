#!/usr/bin/env sh
set -eu

project_name="gitflect"
bin_dir="${BIN_DIR:-$HOME/.local/bin}"
binary="$bin_dir/$project_name"

info() {
    printf '%s\n' "  $*"
}

err() {
    printf '%s\n' "error: $*" >&2
    exit 1
}

remove_block() {
    profile="$1"
    [ -f "$profile" ] || return 0
    if ! grep -q "# >>> gitflect >>>" "$profile"; then
        return 0
    fi
    tmp="$(mktemp)"
    awk '/# >>> gitflect >>>/{skip=1} skip{if(/# <<< gitflect <<</){skip=0; next}; next} {print}' "$profile" > "$tmp"
    cp "$tmp" "$profile"
    rm -f "$tmp"
    info "removed gitflect block from $profile"
}

printf '\n'
printf 'Removing gitflect...\n'
printf '\n'

if [ -f "$binary" ]; then
    rm -f "$binary"
    info "removed $binary"
else
    info "binary not found at $binary — already removed or installed elsewhere"
fi

remove_block "$HOME/.bashrc"
remove_block "$HOME/.zshrc"

printf '\n'
printf 'gitflect has been removed.\n'
printf '\n'
printf 'Reload your shell to complete the removal:\n'
printf '\n'
printf '  exec $SHELL\n'
printf '\n'
printf 'Or start a new terminal session.\n'
printf '\n'
