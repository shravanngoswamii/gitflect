#!/usr/bin/env sh
set -eu

project_name="gitflect"
default_repo="shravangoswami/gitflect"

repo="${GITFLECT_REPO:-$default_repo}"
version="${GITFLECT_VERSION:-latest}"
base_url="${GITFLECT_BASE_URL:-}"
bin_dir="${BIN_DIR:-$HOME/.local/bin}"
shell_name="auto"
profile_path=""
modify_profile="1"

info() {
    printf '%s\n' "info: $*"
}

warn() {
    printf '%s\n' "warn: $*" >&2
}

err() {
    printf '%s\n' "error: $*" >&2
    exit 1
}

has() {
    command -v "$1" >/dev/null 2>&1
}

usage() {
    cat <<USAGE
gitflect installer

Downloads a GitHub release archive and installs gitflect.
Re-run this script later to update to the newest release.

Usage:
  install.sh [options]

Options:
      --repo OWNER/REPO       GitHub repository [default: $repo]
      --version VERSION       Release version, for example v0.1.0 [default: latest]
      --base-url URL          Override release asset base URL
  -b, --bin-dir DIR           Install directory [default: $bin_dir]
      --shell bash|zsh|none   Shell profile integration [default: auto]
      --profile PATH          Shell profile to update
      --no-modify-profile     Install only the binary
  -h, --help                  Print help

Environment:
  GITFLECT_REPO       Default GitHub repository
  GITFLECT_VERSION    Default release version
  GITFLECT_BASE_URL   Default release asset base URL
  BIN_DIR                     Default install directory
USAGE
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --repo)
            [ "$#" -ge 2 ] || err "--repo requires OWNER/REPO"
            repo="$2"
            shift 2
            ;;
        --repo=*)
            repo="${1#*=}"
            shift
            ;;
        --version)
            [ "$#" -ge 2 ] || err "--version requires a release version"
            version="$2"
            shift 2
            ;;
        --version=*)
            version="${1#*=}"
            shift
            ;;
        --base-url)
            [ "$#" -ge 2 ] || err "--base-url requires a URL"
            base_url="$2"
            shift 2
            ;;
        --base-url=*)
            base_url="${1#*=}"
            shift
            ;;
        -b|--bin-dir)
            [ "$#" -ge 2 ] || err "--bin-dir requires a directory"
            bin_dir="$2"
            shift 2
            ;;
        --bin-dir=*)
            bin_dir="${1#*=}"
            shift
            ;;
        --shell)
            [ "$#" -ge 2 ] || err "--shell requires bash, zsh, or none"
            shell_name="$2"
            shift 2
            ;;
        --shell=*)
            shell_name="${1#*=}"
            shift
            ;;
        --profile)
            [ "$#" -ge 2 ] || err "--profile requires a path"
            profile_path="$2"
            shift 2
            ;;
        --profile=*)
            profile_path="${1#*=}"
            shift
            ;;
        --no-modify-profile)
            modify_profile="0"
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            err "unknown option: $1"
            ;;
    esac
done

detect_target() {
    os="$(uname -s | tr '[:upper:]' '[:lower:]')"
    arch="$(uname -m | tr '[:upper:]' '[:lower:]')"

    case "$arch" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *) err "unsupported CPU architecture: $arch" ;;
    esac

    case "$os" in
        linux) printf '%s-unknown-linux-gnu' "$arch" ;;
        darwin) printf '%s-apple-darwin' "$arch" ;;
        *) err "unsupported operating system: $os" ;;
    esac
}

download() {
    file="$1"
    url="$2"

    if has curl; then
        curl --fail --silent --show-error --location --output "$file" "$url"
    elif has wget; then
        wget --quiet --output-document="$file" "$url"
    else
        err "need curl or wget to download release assets"
    fi
}

release_url() {
    asset="$1"
    if [ -n "$base_url" ]; then
        printf '%s/%s' "${base_url%/}" "$asset"
        return
    fi

    case "$version" in
        latest)
            printf 'https://github.com/%s/releases/latest/download/%s' "$repo" "$asset"
            ;;
        v*)
            printf 'https://github.com/%s/releases/download/%s/%s' "$repo" "$version" "$asset"
            ;;
        *)
            printf 'https://github.com/%s/releases/download/v%s/%s' "$repo" "$version" "$asset"
            ;;
    esac
}

detect_shell() {
    case "$shell_name" in
        bash|zsh|none) printf '%s' "$shell_name" ;;
        auto)
            current_shell="$(basename "${SHELL:-}")"
            case "$current_shell" in
                bash|zsh) printf '%s' "$current_shell" ;;
                *) printf '%s' "none" ;;
            esac
            ;;
        *) err "unsupported shell: $shell_name" ;;
    esac
}

default_profile_for_shell() {
    case "$1" in
        bash) printf '%s/.bashrc' "$HOME" ;;
        zsh) printf '%s/.zshrc' "$HOME" ;;
        none) printf '%s' "" ;;
    esac
}

escape_double_quotes() {
    printf '%s' "$1" | sed 's/["\\$`]/\\&/g'
}

profile_dirname() {
    path="$1"
    case "$path" in
        */*) printf '%s' "${path%/*}" ;;
        *) printf '%s' "." ;;
    esac
}

update_profile() {
    selected_shell="$1"
    [ "$modify_profile" = "1" ] || return 0
    [ "$selected_shell" != "none" ] || return 0

    profile="$profile_path"
    if [ -z "$profile" ]; then
        profile="$(default_profile_for_shell "$selected_shell")"
    fi
    [ -n "$profile" ] || return 0

    mkdir -p "$(profile_dirname "$profile")"
    if [ -f "$profile" ] && grep -q "# >>> gitflect >>>" "$profile"; then
        info "profile already has gitflect integration: $profile"
        return 0
    fi

    escaped_bin_dir="$(escape_double_quotes "$bin_dir")"

    {
        printf '\n'
        printf '%s\n' '# >>> gitflect >>>'
        printf '%s\n' '# Added by the gitflect installer.'
        printf 'export PATH="%s:$PATH"\n' "$escaped_bin_dir"
        printf 'eval "$(gitflect init %s)"\n' "$selected_shell"
        printf '%s\n' '# <<< gitflect <<<'
    } >> "$profile"

    info "added gitflect integration to $profile"
}

target="$(detect_target)"
asset="${project_name}-${target}.tar.gz"
url="$(release_url "$asset")"

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/gitflect.XXXXXX")"
trap 'rm -rf "$tmp_dir"' EXIT INT TERM

archive="$tmp_dir/$asset"
info "downloading $url"
download "$archive" "$url"

info "extracting $asset"
tar -xzf "$archive" -C "$tmp_dir"

binary_path="$tmp_dir/${project_name}-${target}/${project_name}"
[ -f "$binary_path" ] || err "release archive does not contain $project_name"

mkdir -p "$bin_dir"
test_file="$bin_dir/.gitflect-write-test"
if ! ( : > "$test_file" ) 2>/dev/null; then
    err "$bin_dir is not writable; choose another --bin-dir or create it with the right permissions"
fi
rm -f "$test_file"

cp "$binary_path" "$bin_dir/$project_name"
chmod 755 "$bin_dir/$project_name"
info "installed $project_name to $bin_dir/$project_name"

selected_shell="$(detect_shell)"
update_profile "$selected_shell"

if ! has "$project_name"; then
    warn "$bin_dir is not on PATH in this shell yet; restart the shell or run: export PATH=\"$bin_dir:\$PATH\""
fi

info "done"
