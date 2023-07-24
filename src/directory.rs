use chrono::{offset::Local, DateTime};

struct Directory {
    id: i16,
    name: String,
    files: Vec<File>,
    size: i64,
    creation_date: Option<DateTime<Local>>,
}

impl Directory {
    static mut DIR_LAST_ID: i16 = 0;

    fn new(name: &String, files: &Vec<File>) -> Self {
        let new_id;
        LAST_ID += 1;
        new_id = LAST_ID;

        Directory {
            id: new_id,
            name,
            files,
            size: files.iter().map(|file| file.size).sum(),
            creation_date: Some(Local::now()),
        }
    }
    
}