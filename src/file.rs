use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map, Value};
use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

pub static FILEPATH: &str = "~/.config/ore/config.json";

fn read_storage(file_path: &Path) -> io::Result<Map<String, Value>> {
    match fs::File::open(file_path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let data: Value =
                serde_json::from_str(&contents).unwrap_or_else(|_| Value::Object(Map::new()));
            Ok(data.as_object().cloned().unwrap_or_else(Map::new))
        }
        Err(_) => Ok(Map::new()), // Return an empty map if file doesn't exist
    }
}

fn write_storage(file_path: &Path, data: &Map<String, Value>) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;
    let contents = serde_json::to_string(data)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn set_key_value<T: Serialize + DeserializeOwned + 'static>(
    file_path: &Path,
    key: &str,
    value: &T,
) -> io::Result<()> {
    let v = serde_json::to_value(value).unwrap();
    let mut data = read_storage(file_path)?;
    data.insert(key.to_string(), v);
    write_storage(file_path, &data)
}

pub fn get_value<T: Serialize + DeserializeOwned + 'static>(
    file_path: &Path,
    key: &str,
) -> io::Result<T> {
    let data = read_storage(file_path)?;
    if let Some(v) = data.get(key) {
        serde_json::from_value(v.clone()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "No value for key"))
    }
}

pub fn ensure_file_path_exists(file_path: &Path) -> io::Result<PathBuf> {
    // Check if the parent directory exists, and if not, create it
    if let Some(parent_dir) = file_path.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
    }

    // Check if the file itself exists, and if not, create it
    match File::open(file_path) {
        Ok(_) => Ok(file_path.to_path_buf()),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            File::create(file_path)?; // This will automatically create the file
            Ok(file_path.to_path_buf())
        }
        Err(e) => Err(e),
    }
}
