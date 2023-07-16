use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_json_file(file_path: &Path) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(HashMap<String, serde_json::Value> = serde_json::from_str(&contents)?)
}

pub fn write_json_file(data: &HashMap<String, serde_json::Value>, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    let json_string = serde_json::to_string_pretty(data)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}
