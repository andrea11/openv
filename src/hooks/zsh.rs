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
// SOLUTION 1: preexec hook -> it runs twice
/* _openv_hook() {
if [[ "$1" == *"openv wrap"* ]]; then
return  # Allow the original command to run
fi

eval "$(openv wrap "$1")"
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec _openv_hook */

// SOLUTION 2: accept-line hook -> Output is not printed
/* _openv_hook () {
if [[ "$BUFFER" != *"openv wrap"* ]]; then
BUFFER="$(openv wrap $BUFFER)"
zle .accept-line
fi
}
zle -N accept-line _openv_hook */

// SOLUTION 3: debug trap
/* _openv_hook() {
  cmd="${history[$HISTCMD]}";
  if [ "$cmd" ]; then
      if [ "$cmd" = "unwrap" ]; then
          print 'Unwrapping command-line';
          trap - DEBUG;
          return;
      fi
      if [ "$handled" != "$HISTCMD;$cmd" ]; then
          handled="$HISTCMD;$cmd";
          eval $(openv wrap $cmd)
          setopt ERR_EXIT;
      fi
  fi
}

wrap() {
  print 'Wrapping command-line';
  trap '_openv_hook' DEBUG;
}

unwrap() {} */

// SOLUTION 4: zle
/* function openv_accept_line() {
     local original_command="$BUFFER"
     local wrapped_command

     # Call your Rust CLI to get the possibly wrapped version
     wrapped_command="$(openv wrap "$original_command")"

     # If the command changed, update the buffer
     if [[ "$wrapped_command" != "$original_command" ]]; then
       BUFFER="$wrapped_command"
     fi

     zle accept-line  # Run the (possibly wrapped) command
   }

   # Register the widget
   zle -N openv_accept_line

   # Bind the widget to the Enter key (RETURN)
   bindkey '^M' openv_accept_line
   bindkey '^J' openv_accept_line  # Sometimes also used for Enter
*/
