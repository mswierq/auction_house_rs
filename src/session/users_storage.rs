use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use std::collections::HashMap;

//TODO: Store it in database
pub struct UsersStorage {
    users: HashMap<String, String>,
}

impl UsersStorage {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn add_user(
        &mut self,
        user: &str,
        password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let None = self.users.get(user) {
            let salt = SaltString::generate(&mut OsRng);
            let hash = Argon2::default().hash_password(password.as_bytes(), &salt);
            if hash.is_err() {
                return Err("Failed to hash password".into());
            }
            self.users
                .insert(user.to_owned(), hash.unwrap().to_string());
            Ok(())
        } else {
            Err("User already exists".into())
        }
    }

    pub fn update_user(
        &mut self,
        user: &str,
        password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(_) = self.users.get(user) {
            let salt = SaltString::generate(&mut OsRng);
            let hash = Argon2::default().hash_password(password.as_bytes(), &salt);
            if hash.is_err() {
                return Err("Failed to hash password".into());
            }
            self.users
                .insert(user.to_owned(), hash.unwrap().to_string());
            Ok(())
        } else {
            Err("User does not exist".into())
        }
    }

    pub fn verify_user(
        &self,
        user: &str,
        password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(hash_str) = self.users.get(user) {
            let hash = PasswordHash::new(&hash_str);
            if hash.is_err() {
                return Err("Failed to get hash password".into());
            }
            let result = Argon2::default().verify_password(password.as_bytes(), &hash.unwrap());
            if result.is_err() {
                return Err("Wrong username or password".into());
            }
            Ok(())
        } else {
            Err("Wrong username or password".into())
        }
    }

    pub fn remove_user(&mut self, user: &str) {
        self.users.remove(user);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_user_and_verify_its_password() {
        let mut users = UsersStorage::new();
        let user = "user";
        let password = "password";
        users.add_user(user, password).unwrap();
        assert!(users.verify_user(user, password).is_ok());
    }

    #[test]
    fn update_user_and_verify_its_password() {
        let mut users = UsersStorage::new();
        let user = "user";
        let password = "password";
        users.add_user(user, password).unwrap();
        assert!(users.verify_user(user, password).is_ok());
        let new_password = "new password";
        users.update_user(user, new_password).unwrap();
        assert!(users.verify_user(user, new_password).is_ok());
    }

    #[test]
    fn add_user_remove_it_and_expect_verification_to_fail() {
        let mut users = UsersStorage::new();
        let user = "user";
        let password = "password";
        users.add_user(user, password).unwrap();
        users.remove_user(user);
        assert!(users.verify_user(user, password).is_err());
    }

    #[test]
    fn add_user_and_verify_wrong_password() {
        let mut users = UsersStorage::new();
        let user = "user";
        let password = "password";
        users.add_user(user, password).unwrap();
        assert!(users.verify_user(user, "wrong password").is_err());
    }
}
