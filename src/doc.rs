pub fn print_usage() {
    println!("Usage: openv <command> [options]");
    println!();
    println!("Available commands:");
    println!("  execute <command>    Execute the specified command wrapped using op");
    println!("  check <command>      Check if the specified command needs to be wrapped");
    println!("  hook <shell>      Print the shell hook for the specified shell");
    println!("  init <shell>      Set up the shell hook for the specified shell");
    println!();
    println!("Supported shells:");
    println!("  bash, zsh, fish");
    println!();
    println!("Examples:");
    println!("  openv hook bash");
    println!("  openv execute 'npm install'");
    println!("  openv check 'npm install'");
    println!("  openv init bash");
    println!();
}
