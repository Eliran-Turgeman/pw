use crate::storage::storage_trait::Storage;

pub fn get_handler(key: &str, storage: &dyn Storage) -> Result<(), Box<dyn std::error::Error>> {
    match storage.get(key.to_string()) {
        Ok(Some(value)) => {
            println!("Value: {}", value);
            Ok(())
        },
        Ok(None) => {
            println!("Key not found");
            Ok(())
        },
        Err(e) => Err(Box::new(e))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct MockStorage {
        should_fail: bool,
        should_return_none: bool
    }

    impl Storage for MockStorage {
        fn get(&self, _: String) -> Result<Option<String>, std::io::Error> {
            if self.should_return_none {
                return Ok(None);
            }
            if self.should_fail {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Mock Error"))
            } else {
                Ok(Some("Mock Value".to_string()))
            }
        }

        fn set(&self, _: String, _: String) -> Result<(), std::io::Error> {
            unimplemented!()
        }

        fn get_all(&self) -> Result<Option<Vec<String>>, std::io::Error> {
            unimplemented!()
        }
    }

    #[test]
    fn get_handler_returns_value() {
        let storage = MockStorage { should_fail: false, should_return_none: false };
        let result = get_handler("test_key", &storage);
        assert!(result.is_ok());
    }

    #[test]
    fn get_handler_key_not_found() {
        let storage = MockStorage { should_fail: false, should_return_none: true };
        let result = get_handler("test_key", &storage);
        assert!(result.is_ok());
    }

    #[test]
    fn get_handler_error() {
        let storage = MockStorage { should_fail: true, should_return_none: false };
        let result = get_handler("test_key", &storage);
        assert!(result.is_err());
    }
}
