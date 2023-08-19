use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, Write};

pub struct Db {
    db_file: File,
}

impl Db {
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::create(format!("{}.json", name))?;
        Ok(Db { db_file: file })
    }

    pub fn select<T>(&mut self, property: &str) -> Result<Option<T>, Box<dyn Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut content = String::new();
        self.db_file.read_to_string(&mut content)?;

        let deserialized: Result<Value, serde_json::Error> = serde_json::from_str(&content);
        match deserialized {
            Ok(data) => {
                let selected_data = data[property].clone();
                let result: Result<T, serde_json::Error> = serde_json::from_value(selected_data);
                match result {
                    Ok(selected) => Ok(Some(selected)),
                    Err(_) => Ok(None),
                }
            }
            Err(_) => Ok(None),
        }
    }

    pub fn insert<T: Serialize>(&mut self, data: &T) -> Result<(), Box<dyn Error>> {
        let serialized_data = serde_json::to_string(data)?;
        self.db_file.write_all(serialized_data.as_bytes())?;
        self.db_file.write_all(b"\n")?;
        Ok(())
    }

    pub fn delete(&mut self, id_to_delete: u32) -> Result<(), Box<dyn Error>> {
        let mut content = String::new();
        self.db_file.read_to_string(&mut content)?;

        let mut parsed_data: Value = serde_json::from_str(&content)?;

        if let Some(index) = parsed_data
            .as_array()
            .and_then(|arr| arr.iter().position(|obj| obj["id"] == id_to_delete))
        {
            parsed_data.as_array_mut().unwrap().remove(index);
        } else {
            println!("Object with id '{}' not found.", id_to_delete);
            return Ok(());
        }

        self.db_file.seek(std::io::SeekFrom::Start(0))?;
        self.db_file.set_len(0)?;

        serde_json::to_writer(&mut self.db_file, &parsed_data)?;
        Ok(())
    }

    pub fn update(&mut self, old_value: &str, new_value: &str) -> Result<(), Box<dyn Error>> {
        let mut content = String::new();
        self.db_file.read_to_string(&mut content)?;

        let mut parsed_data: Value = serde_json::from_str(&content)?;

        if parsed_data[old_value].is_null() {
            println!("Value '{}' does not exist.", old_value);
            return Ok(());
        } else {
            parsed_data[old_value] = Value::String(new_value.to_string());
        }

        self.db_file.seek(std::io::SeekFrom::Start(0))?;
        self.db_file.set_len(0)?;

        serde_json::to_writer(&mut self.db_file, &parsed_data)?;
        Ok(())
    }
}
