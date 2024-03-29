pub trait Storage {
    fn set(&self, key: String, value: String) -> Result<(), std::io::Error>;
    fn get(&self, key: String) -> Result<Option<String>, std::io::Error>;
    fn get_all(&self) -> Result<Option<Vec<String>>, std::io::Error>;
}
