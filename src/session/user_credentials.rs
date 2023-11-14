pub mod memory_storage;

pub trait UserCredentials {
    fn add_user(&mut self, user: &str, password: &str) -> Result<(), Box<dyn std::error::Error>>;

    fn update_user(&mut self, user: &str, password: &str)
        -> Result<(), Box<dyn std::error::Error>>;

    fn verify_user(&self, user: &str, password: &str) -> Result<(), Box<dyn std::error::Error>>;

    fn remove_user(&mut self, user: &str);
}
