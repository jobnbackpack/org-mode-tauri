use std::{fs, path::Path, error::Error};

pub fn read_file(path: &Path) -> Result<String, Box<dyn Error>> {
    let result = fs::read_to_string(path)?.parse()?;
    Ok(result)
}
