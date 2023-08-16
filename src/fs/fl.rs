use chrono::{offset::Local, DateTime};


#[derive(Debug, Clone)]
pub struct File {
    id: i16,
    name: String,
    extension: String,
    pub size: i64,
    creation_date: Option<DateTime<Local>>,
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.extension == other.extension && self.size == other.size
    }
}

impl File {
    pub fn new(name: &str, extension: &str, size: i64) -> Self {

        File {
            id: File::get_next_id(),
            name: name.to_string(),
            extension: extension.to_string(),
            size,
            creation_date: Some(Local::now()),
        }
    }

    fn get_next_id() -> i16 {
        static mut FL_LAST_ID: i16 = 0;
        unsafe {
            FL_LAST_ID += 1;
            FL_LAST_ID
        }
    }
}

