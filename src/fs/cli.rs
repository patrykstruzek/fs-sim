use crate::fs::FilSys;
use crate::new_json_db;
use crate::usr::User;
use std::fs::File;

struct Session {
    filsys: FilSys,
    user: User,
    db: File
}

impl Session {
    fn new() -> Self {
        Session {
            filsys: FilSys::new(),
            user: User::new(),
            db: new_json_db("db").unwrap()
        }
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}