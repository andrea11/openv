pub const BASH_HOOK: &str = r#"
_openv_hook() {
  [[ "$BASH_COMMAND" == *"openv wrap"* ]] && return # Avoid recursion
  [[ $- != *i* ]] && return  # Only run in interactive shells

  cmd="$(openv wrap "$BASH_COMMAND" 2>/dev/null)"
  eval "$cmd"
  history -s "$cmd"
  trap - DEBUG
}

trap '_openv_hook' DEBUG
"#;
