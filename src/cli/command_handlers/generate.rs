use crate::{password_generator::generator::generate_strong_password, storage::storage_trait::Storage};

pub fn generate_handler(
    key: Option<String>,
    length: usize,
    storage: &dyn Storage,
) -> Result<(), Box<dyn std::error::Error>> {
    let password = generate_strong_password(length);
    if let Some(value) = key {
        storage.set(value.clone(), password.clone())?;
        println!("Password generated: {}, and saved under key '{}'", password, value);
    } else {
        println!("Password generated: {}", password);
    }
    Ok(())
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
    fn generate_handler_with_key_saves_password() {
        let mock_storage = MockStorage { should_fail: false };
        let key = Some("test_key".to_string());
        let length = 10;

        let result = generate_handler(key, length, &mock_storage);
        assert!(result.is_ok());
    }

    #[test]
    fn generate_handler_without_key_generates_password() {
        let mock_storage = MockStorage { should_fail: false };
        let length = 10;

        let result = generate_handler(None, length, &mock_storage);
        assert!(result.is_ok());
    }

    #[test]
    fn generate_handler_storage_failure() {
        let mock_storage = MockStorage { should_fail: true };
        let key = Some("test_key".to_string());
        let length = 10;

        let result = generate_handler(key, length, &mock_storage);
        assert!(result.is_err());
    }
}