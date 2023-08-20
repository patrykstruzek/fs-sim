use crate::db::Db;
use crate::fs::FilSys;
use crate::usr::{User, Name, Input};

pub struct Session {
    filsys: FilSys,
    curr_user: User,
}

impl Session {
    pub fn new() -> Self {
        Session {
            filsys: FilSys::new(),
            curr_user: User::new(),
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Register:");
        let new_usr = User::new();
        let mut db = Db::new("db").unwrap();
        db.insert(&new_usr).unwrap();
        self.curr_user = new_usr;
        self.curr_user.name = Name::input();
        db.update("1", &self.curr_user).unwrap();
        Ok(())
    }
}
