use crate::fs::FilSys;
use crypto_hash::{Algorithm, hex_digest};
use regex::Regex;

use std::io;

struct Session {
    filsys: FilSys,
    user: User
}

impl Session {
    fn new() -> Self {
        Session {
            filsys: FilSys::new(),
            user: User::new()
        }
    }
    
    fn run() {

    }
}

struct User {
    id: u16,
    name: Name
}

impl User {
    fn new() -> Self {
        let login = Login::new();
        User { id: 1, name: login.name }
    }

    fn delete() {

    }

    fn update() {

    }
}

struct Login {
    name: Name,
    password: Password
}

impl Login {
    fn new() -> Self {
        println!("name: ");
        let name = Name::input();
        println!("password: ");
        let password = Password::input();
        Login {
            name,
            password
        }
    }

    fn change<T: Input>(input: T , new: &mut str) {

    }
}

trait Input {
    fn input() -> Self;
}

struct Name(String);
impl Input for Name {
    fn input() -> Self {
        loop {
            let mut name = String::new();
            match io::stdin().read_line(&mut name) {
                Ok(_) => {
                    let trimmed_name = name.trim();
                    if trimmed_name.len() >= 1 && trimmed_name.len() <= 12 {
                        return Name(trimmed_name.to_string());
                    } else {
                        println!("Name must have between 1 and 12 characters.");
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                }
            }
        }
    }
}

struct Password(String);
impl Input for Password {
    fn input() -> Self {
        loop {
            let mut passwd = String::new();
            match io::stdin().read_line(&mut passwd) {
                Ok(_) => {
                    let password_regex = Regex::new(r"^(?=.*\d)(?=.*[A-Z])(?=.*\W).{8,}$").unwrap();
                    if password_regex.is_match(&passwd) {
                        let hashed_password = hex_digest(Algorithm::SHA256, passwd.as_bytes());
                        return Password(hashed_password);
                    } else {
                        println!("Invalid password. It must have at least 8 characters,
                         one digit, one uppercase letter, and one special character.");
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                }
            }
        }
    }
}