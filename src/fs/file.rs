use chrono::{offset::Local, DateTime};

pub struct File {
    id: i16,
    name: String,
    extension: String,
    pub size: i64,
    creation_date: Option<DateTime<Local>>,
}

impl File {
    pub fn new(name: String, extension: String, size: i64) -> Self {

        File {
            id: File::get_next_id(),
            name,
            extension,
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
