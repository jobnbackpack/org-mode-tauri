use std::{fs, path::Path, error::Error};

pub fn read_file(path: &Path) -> Result<String, Box<dyn Error>> {
    let result = fs::read_to_string(path)?.parse()?;
    Ok(result)
}

pub fn read_dir(path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result: Vec<String> = Vec::new();
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.as_ref().unwrap();
        match path.path().extension() {
            Some(ex) => {
                if ex.to_str().unwrap() == "org" {
                    println!("path: {}", path.path().display());
                    result.push(fs::read_to_string(path.path())?.parse()?);
                } else {
                    println!("other file extension");
                }
            },
            None => { println!("no extension"); }
        };
    };
    Ok(result)
}
