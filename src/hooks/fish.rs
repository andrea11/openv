pub const FISH_HOOK: &str = r#"
function fish_preexec --on-event fish_preexec
    if string match -q '*openv wrap*' -- "$argv[1]"
        return
    end

    set wrapped (openv wrap "$argv[1]")

    # Only run wrapped if it differs from the original
    if test -n "$wrapped"
        eval $wrapped
        commandline --replace ''
        commandline --function repaint
        return 1  # Cancel the original command
    end
end
"#;
