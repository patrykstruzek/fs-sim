use chrono::{offset::Local, DateTime};

struct File {
    id: i16,
    name: String,
    extension: String,
    size: i64,
    creation_date: Option<DateTime<Local>>,
}

impl File {
    static mut FL_LAST_ID: i16 = 0;

    fn new(name: &String, extension: &String, size: &i64) -> Self {
        let new_id;
        LAST_ID += 1;
        new_id = LAST_ID;

        File {
            id: new_id,
            name,
            extension,
            size,
            creation_date: Some(Local::now()),
        }
    }
}
