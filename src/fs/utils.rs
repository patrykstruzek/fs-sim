use std::fs::File;

pub fn new_json_db(name: &str) -> Result<File, Box<dyn std::error::Error>> {
        let file = File::create(format!("{}.json", name))?;
        Ok(file)
}