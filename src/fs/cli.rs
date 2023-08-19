use crate::fs::FilSys;
use crate::db::Db;
use crate::usr::User;
pub struct Session {
    filsys: FilSys,
    curr_user: User,
    db: Db,
}

impl Session {
    pub fn new() -> Self {
        Session {
            filsys: FilSys::new(),
            curr_user: User::new(),
            db: Db::new("db").unwrap(),
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
            println!("Register:");
            let new_usr = User::new();
            self.db.insert(&new_usr).unwrap();
            self.curr_user = new_usr;
        Ok(())
    }
}