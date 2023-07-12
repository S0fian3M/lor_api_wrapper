use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_json_file(path: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_json_file(data: &Value, path: &Path) -> Result<(), std::io::Error> {
    let file = File::create(path)?;
    serde_json::to_writer(file, data)?;
    Ok(())
}
