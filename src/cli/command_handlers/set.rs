use crate::storage::storage_trait::Storage;

pub fn set_handler(key: &str, value: &str, storage: &dyn Storage) -> Result<(), Box<dyn std::error::Error>> {
    storage.set(key.to_string(), value.to_string())
        .map_err(|e| e.into())
        .map(|_| println!("Key set successfully"))
}


#[cfg(test)]
mod tests {
    use super::*;

    struct MockStorage {
        should_fail: bool,
    }

    impl Storage for MockStorage {
        fn set(&self, _key: String, _value: String) -> Result<(), std::io::Error> {
            if self.should_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Mock Error"))
            } else {
                Ok(())
            }
        }
        fn get(&self, _: String) -> Result<Option<String>, std::io::Error> {
            unimplemented!()        }
    
        fn get_all(&self) -> Result<Option<Vec<String>>, std::io::Error> {
            unimplemented!()        }
    }

    #[test]
    fn test_set_handler_failure() {
        let storage = MockStorage { should_fail: true };
        let result = set_handler("test_key", "test_value", &storage);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Mock Error");
    }
}
