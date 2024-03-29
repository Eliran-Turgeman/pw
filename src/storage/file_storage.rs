use super::storage_trait::Storage;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct FileStorage {
    file_path: String,
}

impl FileStorage {
    pub fn new(file_path: &str) -> Self {
        let path = Path::new(file_path);

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                match fs::create_dir_all(parent) {
                    Ok(_) => println!("Directory created: {:?}", parent),
                    Err(e) => panic!("Failed to create directory: {:?}, error: {}", parent, e),
                }
            }
        }

        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
        {
            Ok(file) => file,
            Err(e) => panic!(
                "Failed to open or create file: {:?}, error: {}",
                file_path, e
            ),
        };

        match file.metadata() {
            Ok(metadata) => {
                if metadata.len() == 0 {
                    if let Err(e) = file.write_all(b"{}") {
                        panic!("Failed to write to file: {:?}, error: {}", file_path, e);
                    }
                }
            }
            Err(e) => panic!("Failed to get file metadata: {:?}, error: {}", file_path, e),
        }

        Self {
            file_path: file_path.to_string(),
        }
    }

    fn read_storage(&self) -> io::Result<HashMap<String, String>> {
        match File::open(&self.file_path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let map: HashMap<String, String> = serde_json::from_str(&contents)?;
                Ok(map)
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(HashMap::new()),
            Err(e) => Err(e),
        }
    }

    fn write_storage(&self, map: &HashMap<String, String>) -> io::Result<()> {
        let contents = serde_json::to_string(map)?;
        fs::write(&self.file_path, contents)
    }
}

impl Storage for FileStorage {
    fn set(&self, key: String, value: String) -> io::Result<()> {
        let mut map = self.read_storage()?;
        map.insert(key, value);
        self.write_storage(&map)
    }

    fn get(&self, key: String) -> io::Result<Option<String>> {
        let map = self.read_storage()?;
        Ok(map.get(&key).cloned())
    }

    fn get_all(&self) -> io::Result<Option<Vec<String>>> {
        let map = self.read_storage()?;
        if map.is_empty() {
            Ok(None)
        } else {
            let values = map.values().cloned().collect::<Vec<String>>();
            Ok(Some(values))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::storage_trait::Storage;
    use std::io;
    use tempfile::tempdir;

    #[test]
    fn test_set_and_get() -> io::Result<()> {
        let temp_dir = tempdir()?; // Create a temporary directory
        let file_path = temp_dir.path().join("store.json");
        let storage = super::FileStorage::new(file_path.to_str().unwrap());

        storage.set("key".to_string(), "value".to_string())?;

        assert_eq!(storage.get("key".to_string())?, Some("value".to_string()));
        Ok(())
    }

    #[test]
    fn test_get_nonexistent_key() -> io::Result<()> {
        let temp_dir = tempdir()?; // Create a temporary directory
        let file_path = temp_dir.path().join("store.json");
        let storage = super::FileStorage::new(file_path.to_str().unwrap());

        assert_eq!(storage.get("nonexistent_key".to_string())?, None);

        Ok(())
    }

    #[test]
    fn test_persistence() -> io::Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("store.json");
        let storage = super::FileStorage::new(file_path.to_str().unwrap());

        storage.set("persisted_key".to_string(), "persisted_value".to_string())?;
        drop(storage);

        let new_storage = super::FileStorage::new(file_path.to_str().unwrap());
        assert_eq!(
            new_storage.get("persisted_key".to_string())?,
            Some("persisted_value".to_string())
        );

        Ok(())
    }

    #[test]
    fn test_get_all() -> io::Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("store.json");
        let storage = super::FileStorage::new(file_path.to_str().unwrap());

        storage.set("k1".to_string(), "v1".to_string())?;
        storage.set("k2".to_string(), "v2".to_string())?;
        storage.set("k3".to_string(), "v3".to_string())?;

        let mut values = storage.get_all()?.unwrap_or_default();
        values.sort();

        let mut expected = vec!["v1".to_string(), "v2".to_string(), "v3".to_string()];
        expected.sort();

        assert_eq!(values, expected);

        Ok(())
    }
}
