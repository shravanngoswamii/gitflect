use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Plain,
    Raw,
    Zsh,
}

impl Shell {
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "bash" => Some(Self::Bash),
            "plain" | "none" | "no-color" => Some(Self::Plain),
            "raw" | "ansi" => Some(Self::Raw),
            "zsh" => Some(Self::Zsh),
            _ => None,
        }
    }
}

impl fmt::Display for Shell {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shell::Bash => formatter.write_str("bash"),
            Shell::Plain => formatter.write_str("plain"),
            Shell::Raw => formatter.write_str("raw"),
            Shell::Zsh => formatter.write_str("zsh"),
        }
    }
}

pub fn init(shell: Shell, binary: &str) -> String {
    match shell {
        Shell::Bash => init_bash(binary),
        Shell::Zsh => init_zsh(binary),
        Shell::Plain | Shell::Raw => "init expects a concrete shell: bash or zsh\n".to_string(),
    }
}

fn init_bash(binary: &str) -> String {
    let binary = shell_quote(binary);
    format!(
        r#"# gitflect Bash integration
__gitflect_bin={binary}
__gitflect_original_ps1=${{PS1-}}
__gitflect_current_ps1=

__gitflect_apply_segment() {{
    local base="$1"
    local segment="$2"
    local marker
    local prefix
    local suffix

    if [[ -z "$segment" ]]; then
        printf '%s' "$base"
        return
    fi

    for marker in '\$' '$' '#' '>'; do
        if [[ "$base" == *"$marker"* ]]; then
            prefix="${{base%"$marker"*}}"
            suffix="${{base#"$prefix"}}"
            printf '%s %s%s' "$prefix" "$segment" "$suffix"
            return
        fi
    done

    printf '%s %s' "$base" "$segment"
}}

__gitflect_prompt() {{
    local __gitflect_status=$?
    local __gitflect_base
    local __gitflect_rendered
    local __gitflect_segment

    if [[ -n "${{__gitflect_current_ps1+x}}" && "$PS1" != "$__gitflect_current_ps1" ]]; then
        __gitflect_original_ps1="$PS1"
    fi

    if [[ "${{GITFLECT_REPLACE_PROMPT:-0}}" == "1" ]]; then
        __gitflect_rendered="$("$__gitflect_bin" prompt --shell bash --last-status "$__gitflect_status")"
    else
        __gitflect_base="${{__gitflect_original_ps1:-$PS1}}"
        __gitflect_segment="$("$__gitflect_bin" prompt --shell bash --status-only --last-status "$__gitflect_status")"
        __gitflect_rendered="$(__gitflect_apply_segment "$__gitflect_base" "$__gitflect_segment")"
    fi

    __gitflect_current_ps1="$__gitflect_rendered"
    PS1="$__gitflect_rendered"
    return "$__gitflect_status"
}}

case ";${{PROMPT_COMMAND:-}};" in
    *";__gitflect_prompt;"*) ;;
    *) PROMPT_COMMAND="${{PROMPT_COMMAND:+$PROMPT_COMMAND;}}__gitflect_prompt" ;;
esac

_gitflect_git_complete() {{
    local __gitflect_cmds
    mapfile -t COMPREPLY < <("$__gitflect_bin" complete --shell bash --position "$COMP_CWORD" -- "${{COMP_WORDS[@]}}")
    compopt -o default 2>/dev/null || true
}}

for __gitflect_cmd in ${{GITFLECT_GIT_COMMANDS:-git}}; do
    complete -o default -F _gitflect_git_complete "$__gitflect_cmd"
done
unset __gitflect_cmd
"#
    )
}

fn init_zsh(binary: &str) -> String {
    let binary = shell_quote(binary);
    format!(
        r#"# gitflect Zsh integration
__gitflect_bin={binary}
typeset -g __gitflect_original_prompt="${{PROMPT-}}"
typeset -g __gitflect_current_prompt=

__gitflect_apply_segment() {{
    local base="$1"
    local segment="$2"
    local marker
    local prefix
    local suffix

    if [[ -z "$segment" ]]; then
        printf '%s' "$base"
        return
    fi

    for marker in '%#' '$' '#' '>'; do
        if [[ "$base" == *"$marker"* ]]; then
            prefix="${{base%"$marker"*}}"
            suffix="${{base#"$prefix"}}"
            printf '%s %s%s' "$prefix" "$segment" "$suffix"
            return
        fi
    done

    printf '%s %s' "$base" "$segment"
}}

__gitflect_precmd() {{
    local __gitflect_status=$?
    local __gitflect_base
    local __gitflect_rendered
    local __gitflect_segment

    if [[ -n "${{__gitflect_current_prompt+x}}" && "$PROMPT" != "$__gitflect_current_prompt" ]]; then
        __gitflect_original_prompt="$PROMPT"
    fi

    if [[ "${{GITFLECT_REPLACE_PROMPT:-0}}" == "1" ]]; then
        __gitflect_rendered="$("$__gitflect_bin" prompt --shell zsh --last-status "$__gitflect_status")"
    else
        __gitflect_base="${{__gitflect_original_prompt:-$PROMPT}}"
        __gitflect_segment="$("$__gitflect_bin" prompt --shell zsh --status-only --last-status "$__gitflect_status")"
        __gitflect_rendered="$(__gitflect_apply_segment "$__gitflect_base" "$__gitflect_segment")"
    fi

    __gitflect_current_prompt="$__gitflect_rendered"
    PROMPT="$__gitflect_rendered"
    return "$__gitflect_status"
}}

autoload -Uz add-zsh-hook
add-zsh-hook precmd __gitflect_precmd

autoload -Uz compinit
if ! whence -w compdef >/dev/null 2>&1; then
    compinit
fi

_gitflect_git_complete() {{
    local -a __gitflect_completions
    __gitflect_completions=("${{(@f)$("$__gitflect_bin" complete --shell zsh --position "$((CURRENT - 1))" -- "${{words[@]}}")}}")
    compadd -- "${{__gitflect_completions[@]}}"
}}

for __gitflect_cmd in ${{=GITFLECT_GIT_COMMANDS:-git}}; do
    compdef _gitflect_git_complete "$__gitflect_cmd"
done
unset __gitflect_cmd
"#
    )
}

fn shell_quote(value: &str) -> String {
    if value.is_empty() {
        return "''".to_string();
    }

    let mut quoted = String::from("'");
    for character in value.chars() {
        if character == '\'' {
            quoted.push_str("'\\''");
        } else {
            quoted.push(character);
        }
    }
    quoted.push('\'');
    quoted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_shell_paths() {
        assert_eq!(shell_quote("/tmp/a b/bin"), "'/tmp/a b/bin'");
        assert_eq!(shell_quote("/tmp/a'b/bin"), "'/tmp/a'\\''b/bin'");
    }
}
