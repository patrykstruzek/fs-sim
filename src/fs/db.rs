use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Debug)]
pub enum DbError {
    IoErr(std::io::Error),
    SerdeErr(serde_json::Error),
    DataDoesNotExist,
}

impl From<std::io::Error> for DbError {
    fn from(err: std::io::Error) -> Self {
        DbError::IoErr(err)
    }
}

impl From<serde_json::Error> for DbError {
    fn from(err: serde_json::Error) -> Self {
        DbError::SerdeErr(err)
    }
}

pub struct Db {
    db_file: File,
}

impl Db {
    pub fn new(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(format!("{}.json", name))?;
        Ok(Db { db_file: file })
    }

    pub fn select<T>(&mut self, id: &str) -> Result<Option<T>, DbError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut content = String::new();
        self.db_file.read_to_string(&mut content)?;

        let deserialized: Value = serde_json::from_str(&content)?;

        let selected_data = match deserialized.get(&id) {
            Some(data) => data.clone(),
            None => return Ok(None),
        };

        let selected = serde_json::from_value(selected_data)?;

        Ok(Some(selected))
    }

    pub fn insert<T: Serialize>(&mut self, data: &T) -> Result<(), DbError> {
        let serialized_data = serde_json::to_string(data)?;
        self.db_file.write_all(serialized_data.as_bytes())?;
        self.db_file.write_all(b"\n")?;
        Ok(())
    }

    pub fn delete(&mut self, id_to_delete: u32) -> Result<(), DbError> {
        let mut content = String::new();
        self.db_file.read_to_string(&mut content)?;

        let mut parsed_data: Value = serde_json::from_str(&content)?;

        if let Some(index) = parsed_data
            .as_array()
            .and_then(|arr| arr.iter().position(|obj| obj["id"] == id_to_delete))
        {
            parsed_data.as_array_mut().unwrap().remove(index);
        } else {
            return Err(DbError::DataDoesNotExist);
        }

        self.db_file.seek(std::io::SeekFrom::Start(0))?;
        self.db_file.set_len(0)?;

        serde_json::to_writer(&mut self.db_file, &parsed_data)?;
        Ok(())
    }

    pub fn update<T: Serialize + Deserialize<'static>>(
        &mut self,
        id: &str,
        data_for_replace: &T,
    ) -> Result<(), DbError> {
        let mut content = String::new();
        self.db_file.seek(SeekFrom::Start(0))?;
        self.db_file.read_to_string(&mut content)?;

        let mut parsed_data: Value = from_str(&content)?;

        if parsed_data[id].is_null() {
            return Err(DbError::DataDoesNotExist);
        } else {
            parsed_data[id] = serde_json::to_value(data_for_replace)?;
        }

        
        self.db_file.set_len(0)?;
        self.db_file.seek(SeekFrom::Start(0))?;
        
        serde_json::to_writer(&mut self.db_file, &parsed_data)?;
        Ok(())
    }
}
