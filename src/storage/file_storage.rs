use super::storage_trait::Storage;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read};

#[derive(Serialize, Deserialize)]
pub struct FileStorage {
    file_path: String,
}

impl FileStorage {
    pub fn new(file_path: &str) -> Self {
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
            },
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
}


#[cfg(test)]
mod tests {
    use std::io;
    use tempfile::tempdir;
    use crate::storage::storage_trait::Storage;

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
        assert_eq!(new_storage.get("persisted_key".to_string())?, Some("persisted_value".to_string()));

        Ok(())
    }

}
