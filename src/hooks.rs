pub mod bash;
pub mod fish;
pub mod zsh;

use crate::hooks::bash::BASH_HOOK;
use crate::hooks::fish::FISH_HOOK;
use crate::hooks::zsh::ZSH_HOOK;

pub fn print_hook(shell: &str) {
    match shell {
        "bash" => {
            println!("{BASH_HOOK}");
        }
        "zsh" => {
            println!("{ZSH_HOOK}");
        }
        "fish" => {
            println!("{FISH_HOOK}");
        }
        _ => {
            eprintln!("Unsupported shell: {shell}");
        }
    }
}
