pub const ZSH_HOOK: &str = r#"
_openv_hook() {
  emulate -L zsh
  setopt localoptions no_aliases

  if openv check "$BUFFER" > /dev/null 2>&1; then
    print -s -- "$BUFFER"

    BUFFER="openv execute ${(q)BUFFER}"
    zle accept-line
    return 0
  fi

  zle .accept-line
}

zle -N _openv_hook

bindkey '^M' _openv_hook
bindkey '^J' _openv_hook
"#;
