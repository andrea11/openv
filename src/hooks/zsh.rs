pub const ZSH_HOOK: &str = r#"
_openv_hook() {
  emulate -L zsh
  setopt localoptions no_aliases

  if [[ "$BUFFER" != *"openv wrap"* ]]; then
    BUFFER="$("openv" wrap "$BUFFER")"
    fi

  zle accept-line
}

zle -N _openv_hook

bindkey '^M' _openv_hook
bindkey '^J' _openv_hook
"#;
