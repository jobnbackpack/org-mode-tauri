use std::{fs, path::Path, error::Error};

pub fn read_file(path: &Path) -> Result<String, Box<dyn Error>> {
    let result = fs::read_to_string(path)?.parse()?;
    Ok(result)
}

pub struct RawFile {
    pub name: String,
    pub path: String,
    pub raw: String
}

pub fn read_dir(path: &Path) -> Result<Vec<RawFile>, Box<dyn Error>> {
    let mut result: Vec<RawFile> = Vec::new();
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.as_ref().unwrap();
        match path.path().extension() {
            Some(ex) => {
                if ex.to_str().unwrap() == "org" {
                    result.push(
                        RawFile {
                            name: path.path().file_name().unwrap().to_owned().to_str().unwrap().to_string(),
                            path: path.path().to_str().unwrap().to_owned().to_string(),
                            raw: fs::read_to_string(path.path())?.parse()?
                        }
                    );
                } else {
                    println!("other file extension");
                }
            },
            None => { println!("no extension"); }
        };
    };
    Ok(result)
}
