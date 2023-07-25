use chrono::{offset::Local, DateTime};

use crate::file::File;

struct Directory {
    id: i16,
    name: String,
    size: i64,
    files: Vec<File>,
    creation_date: Option<DateTime<Local>>,
}

impl Directory {
    fn new(name: &str, files: Vec<File>) -> Self {
        Directory {
            id: Directory::get_next_id(),
            name: name.to_string(),
            size: Directory::sum_size(&files),
            files,
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

    fn sum_size(files: &Vec<File>) -> i64 {
        files.iter().map(|file| file.size).sum()
    }
    
}