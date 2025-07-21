pub const FISH_HOOK: &str = r#"
function _openv_hook
    set cmdline (commandline)

    if openv check "$cmdline" > /dev/null 2>&1
        openv execute "$cmdline"
        history merge
        history append "$cmdline"
        history save
        commandline ''
    else
        commandline -f execute
    end
end

bind \r _openv_hook
bind \cj _openv_hook
"#;
