use crypto_hash::{Algorithm, hex_digest};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: u16,
    pub name: Name
}

impl User {
    pub fn new() -> Self {
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
        let name = Name::input();
        let password = Password::input();
        Login {
            name,
            password
        }
    }

    fn change<T: Input>(name: &str) {
        T::input();
    }
}

pub trait Input {
    fn input() -> Self;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Name(String);
impl Input for Name {
    fn input() -> Self {
        loop {
            super::print("name: ");
            std::io::stdout().flush().unwrap();
            let mut name = String::new();
            match io::stdin().read_line(&mut name) {
                Ok(_) => {
                    let trimmed_name = name.trim();
                    if Self::is_valid(&trimmed_name) {
                        return Name(trimmed_name.to_string());
                    } else {
                        super::clr_curr_ln();
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

impl Name {
    fn is_valid(name: &str) -> bool {
        name.len() >= 1 && name.len() <= 12
    }
}

struct Password(String);
impl Input for Password {
    fn input() -> Self {
        loop {
            super::print("password: ");
            let mut passwd = String::new();
            match io::stdin().read_line(&mut passwd) {
                Ok(_) => {
                    if Self::is_valid(&passwd) {
                        let hashed_passwd
                        = hex_digest(Algorithm::SHA256, passwd.as_bytes());
                        return Password(hashed_passwd);
                    } else {
                        super::clr_curr_ln();
                        println!("Invalid password. It must have at least:\n\
                        8 characters,\n\
                        one digit,\n\
                        one uppercase letter,\n\
                        and one special character.");
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                }
            }
        }
    }
}

impl Password {
    pub fn is_valid(password: &str) -> bool {
        let length_condition = password.len() >= 8;
        let uppercase_condition = password.chars().any(|c| c.is_ascii_uppercase());
        let lowercase_condition = password.chars().any(|c| c.is_ascii_lowercase());
        let digit_condition = password.chars().any(|c| c.is_ascii_digit());
        let special_character_condition = regex::Regex::new(r"[@#$%^&+=!*_]").unwrap().is_match(password);
    
        length_condition && uppercase_condition && lowercase_condition && digit_condition && special_character_condition
    }
}