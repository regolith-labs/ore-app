use directories::ProjectDirs;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map, Value};
use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};
use std::path::PathBuf;

fn filepath() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "ore", "app") {
        // This gives you a platform-specific config directory
        // For example, on Windows, this might be C:\Users\Username\AppData\Roaming\YourCompany\YourApp\config
        // On Linux, /home/username/.config/YourApp, and on macOS, /Users/username/Library/Application Support/com.YourCompany.YourApp
        proj_dirs.config_dir().join("config.json")
    } else {
        panic!("Can't load project directory")
    }
}

fn read_storage() -> io::Result<Map<String, Value>> {
    match fs::File::open(filepath().as_path()) {
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

fn write_storage(data: &Map<String, Value>) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filepath().as_path())?;
    let contents = serde_json::to_string(data)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn set_key_value<T: Serialize + DeserializeOwned + 'static>(
    key: &str,
    value: &T,
) -> io::Result<()> {
    ensure_filepath_exists().ok();
    let v = serde_json::to_value(value).unwrap();
    let mut data = read_storage()?;
    data.insert(key.to_string(), v);
    write_storage(&data)
}

pub fn get_value<T: Serialize + DeserializeOwned + 'static>(key: &str) -> io::Result<T> {
    ensure_filepath_exists().ok();
    let data = read_storage()?;
    if let Some(v) = data.get(key) {
        serde_json::from_value(v.clone()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "No value for key"))
    }
}

fn ensure_filepath_exists() -> io::Result<PathBuf> {
    // Check if the parent directory exists, and if not, create it
    let fp = filepath();
    if let Some(parent_dir) = fp.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
    }

    // Check if the file itself exists, and if not, create it
    match File::open(fp.as_path()) {
        Ok(_) => Ok(fp),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            File::create(fp.as_path())?; // This will automatically create the file
            Ok(fp)
        }
        Err(e) => Err(e),
    }
}
