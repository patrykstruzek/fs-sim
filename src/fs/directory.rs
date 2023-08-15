use std::{cell::{RefCell, Ref}, rc::{Weak, Rc}};

use chrono::{offset::Local, DateTime};

use crate::file::File;


pub struct Directory {
    id: i16,
    name: String,
    size: i64,
    parent_dir: RefCell<Weak<Directory>>,
    children_dir: RefCell<Vec<Rc<Directory>>>,
    files: RefCell<Vec<Rc<File>>>,
    creation_date: Option<DateTime<Local>>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Directory {
            id: Directory::get_next_id(),
            name: name.to_string(),
            size: 0,
            parent_dir: RefCell::new(Weak::new()),
            children_dir: RefCell::new(vec![]),
            files: RefCell::new(vec![]),
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

    fn sum_size(&self) -> i64 {
        self.files.borrow_mut().iter().map(|file| file.size).sum()
    }

    pub fn store(&mut self, file: File) {
        self.files.borrow_mut().push(Rc::new(file));
    }

    pub fn files(&self) -> &RefCell<Vec<Rc<File>>> {
        &self.files
    }
    
}