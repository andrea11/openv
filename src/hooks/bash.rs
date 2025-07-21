pub const BASH_HOOK: &str = r#"
_openv_hook() {
  [[ $- != *i* ]] && return

  if openv check "$BASH_COMMAND" >/dev/null 2>&1; then
    openv execute "$BASH_COMMAND"

    history -s "$BASH_COMMAND"
    trap - DEBUG
    kill -SIGINT $$
  fi
}

trap '_openv_hook' DEBUG
"#;
