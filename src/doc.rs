pub fn print_usage() {
    println!("Usage: openv <command> [options]");
    println!();
    println!("Available commands:");
    println!(
        "  wrap <command>    Wrap the specified command with additional functionality (only if specified in the config)"
    );
    println!("  hook <shell>      Print the shell hook for the specified shell");
    println!("  init <shell>      Set up the shell hook for the specified shell");
    println!();
    println!("Supported shells:");
    println!("  bash, zsh, fish");
    println!();
    println!("Examples:");
    println!("  openv hook bash");
    println!("  openv wrap 'npm install'");
    println!("  openv init bash");
    println!();
}
