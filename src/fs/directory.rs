use chrono::{offset::Local, DateTime};

use crate::file::File;

pub struct Directory {
    id: i16,
    name: String,
    size: i64,
    files: Vec<File>,
    creation_date: Option<DateTime<Local>>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Directory {
            id: Directory::get_next_id(),
            name: name.to_string(),
            size: 0,
            files: vec![],
            creation_date: Some(Local::now()),
        }
    }

    fn get_next_id() -> i16 {
        static mut DIR_LAST_ID: i16 = 0;
        unsafe {
            DIR_LAST_ID += 1;
            DIR_LAST_ID
        }
    }

    fn sum_size(files: &[File]) -> i64 {
        files.iter().map(|file| file.size).sum()
    }

    pub fn store(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn files(&self) -> &Vec<File> {
        &self.files
    }
    
}