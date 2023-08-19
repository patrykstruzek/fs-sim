pub mod cli;
pub mod dir;
pub mod fl;
pub mod fs;
pub mod usr;
pub mod db;

// utils:

use std::io::*;

pub fn clr_curr_ln() {
    print!("\x1B[2K");
    print!("\x1B[1F");
    std::io::stdout().flush().unwrap();
}

pub fn print(text: &str) {
    print!("{}", text);
    std::io::stdout().flush().unwrap();
}