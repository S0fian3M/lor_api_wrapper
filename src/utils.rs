use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;

/**
* Read JSON file to HashMap.
*/
pub fn read_json_file(json_file: &Path) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut file = File::open(json_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let json_value: serde_json::Value = serde_json::from_str(&contents)?;
    Ok(json_value)
}

/**
* Write HashMap to JSON file.
*/
fn write_json_file(json_file: &str, data: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let serialized_data = serde_json::to_string(data)?;
    let mut file = File::create(json_file)?;
    file.write_all(serialized_data.as_bytes())?;
    Ok(())
}
