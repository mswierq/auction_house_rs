use crate::backend::Funds;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Default)]
struct UserData {
    funds: Funds,
    items: HashSet<String>,
}

#[derive(Default)]
pub struct UsersMemoryStorage {
    users: HashMap<String, UserData>,
}

impl super::UsersBackend for UsersMemoryStorage {
    fn add_user(&mut self, user: &str) -> Result<(), Box<dyn Error>> {
        if self.users.contains_key(user) {
            return Err("User already exists".into());
        }
        self.users.insert(user.to_owned(), UserData::default());
        Ok(())
    }

    fn deposit_funds(&mut self, user: &str, amount: Funds) -> Result<(), Box<dyn Error>> {
        if self.users.contains_key(user) {
            let user_data = self.users.get_mut(user).unwrap();
            if user_data.funds > Funds::MAX - amount {
                return Err("Max funds exceeded".into());
            }
            user_data.funds += amount;
            Ok(())
        } else {
            Err("User does not exist".into())
        }
    }

    fn withdraw_funds(&mut self, user: &str, amount: Funds) -> Result<(), Box<dyn Error>> {
        if self.users.contains_key(user) {
            let user_data = self.users.get_mut(user).unwrap();
            if user_data.funds < amount {
                return Err("Insufficient funds".into());
            }
            user_data.funds -= amount;
            Ok(())
        } else {
            Err("User does not exist".into())
        }
    }

    fn deposit_item(&mut self, user: &str, item: &str) -> Result<(), Box<dyn Error>> {
        if self.users.contains_key(user) {
            let user_data = self.users.get_mut(user).unwrap();
            if user_data.items.contains(item) {
                return Err("Item already exists".into());
            }
            user_data.items.insert(item.to_owned());
            Ok(())
        } else {
            Err("User does not exist".into())
        }
    }

    fn withdraw_item(&mut self, user: &str, item: &str) -> Result<(), Box<dyn Error>> {
        if self.users.contains_key(user) {
            let user_data = self.users.get_mut(user).unwrap();
            if !user_data.items.contains(item) {
                return Err("Item does not exist".into());
            }
            user_data.items.remove(item);
            Ok(())
        } else {
            Err("User does not exist".into())
        }
    }

    fn list_items(&self, user: &str) -> Result<Vec<String>, Box<dyn Error>> {
        if self.users.contains_key(user) {
            let user_data = self.users.get(user).unwrap();
            Ok(user_data.items.iter().cloned().collect())
        } else {
            Err("User does not exist".into())
        }
    }

    fn show_funds(&self, user: &str) -> Result<u32, Box<dyn Error>> {
        if self.users.contains_key(user) {
            let user_data = self.users.get(user).unwrap();
            Ok(user_data.funds)
        } else {
            Err("User does not exist".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::backend::UsersBackend;

    #[test]
    fn test_add_user() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        assert!(storage.users.contains_key("user1"));
    }

    #[test]
    fn test_add_user_twice() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        assert!(storage.add_user("user1").is_err());
    }

    #[test]
    fn test_deposit_funds() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        storage.deposit_funds("user1", 100).unwrap();
        assert_eq!(storage.users.get("user1").unwrap().funds, 100);
    }

    #[test]
    fn test_deposit_funds_twice() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        storage.deposit_funds("user1", 100).unwrap();
        storage.deposit_funds("user1", 100).unwrap();
        assert_eq!(storage.users.get("user1").unwrap().funds, 200);
    }

    #[test]
    fn test_deposit_funds_to_non_existing_user() {
        let mut storage = UsersMemoryStorage::default();
        assert!(storage.deposit_funds("user1", 100).is_err());
    }

    #[test]
    fn test_deposit_funds_exceeding_max() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        storage.deposit_funds("user1", 100).unwrap();
        assert!(storage.deposit_funds("user1", Funds::MAX).is_err());
    }

    #[test]
    fn test_withdraw_funds() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        storage.deposit_funds("user1", 100).unwrap();
        storage.withdraw_funds("user1", 50).unwrap();
        assert_eq!(storage.users.get("user1").unwrap().funds, 50);
    }

    #[test]
    fn test_withdraw_funds_twice() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        storage.deposit_funds("user1", 100).unwrap();
        storage.withdraw_funds("user1", 50).unwrap();
        storage.withdraw_funds("user1", 50).unwrap();
        assert_eq!(storage.users.get("user1").unwrap().funds, 0);
    }

    #[test]
    fn test_withdraw_funds_from_non_existing_user() {
        let mut storage = UsersMemoryStorage::default();
        assert!(storage.withdraw_funds("user1", 100).is_err());
    }

    #[test]
    fn test_withdraw_funds_exceeding_max() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        assert!(storage.withdraw_funds("user1", Funds::MAX).is_err());
    }

    #[test]
    fn test_withdraw_funds_exceeding_balance() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        assert!(storage.withdraw_funds("user1", 100).is_err());
    }

    #[test]
    fn test_deposit_item() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        storage.deposit_item("user1", "item1").unwrap();
        assert!(storage.users.get("user1").unwrap().items.contains("item1"));
    }

    #[test]
    fn test_deposit_item_twice() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        storage.deposit_item("user1", "item1").unwrap();
        assert!(storage.deposit_item("user1", "item1").is_err());
    }

    #[test]
    fn test_deposit_item_to_non_existing_user() {
        let mut storage = UsersMemoryStorage::default();
        assert!(storage.deposit_item("user1", "item1").is_err());
    }

    #[test]
    fn test_withdraw_item() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        storage.deposit_item("user1", "item1").unwrap();
        storage.withdraw_item("user1", "item1").unwrap();
        assert!(!storage.users.get("user1").unwrap().items.contains("item1"));
    }

    #[test]
    fn test_withdraw_item_twice() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        storage.deposit_item("user1", "item1").unwrap();
        storage.withdraw_item("user1", "item1").unwrap();
        assert!(storage.withdraw_item("user1", "item1").is_err());
    }

    #[test]
    fn test_withdraw_item_from_non_existing_user() {
        let mut storage = UsersMemoryStorage::default();
        assert!(storage.withdraw_item("user1", "item1").is_err());
    }

    #[test]
    fn test_list_items() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        storage.deposit_item("user1", "item1").unwrap();
        storage.deposit_item("user1", "item2").unwrap();
        let items = storage.list_items("user1").unwrap();
        assert!(items.contains(&"item1".to_owned()));
        assert!(items.contains(&"item2".to_owned()));
    }

    #[test]
    fn test_show_funds() {
        let mut storage = UsersMemoryStorage::default();
        storage.add_user("user1").unwrap();
        storage.deposit_funds("user1", 100).unwrap();
        assert_eq!(storage.show_funds("user1").unwrap(), 100);
    }

    #[test]
    fn test_show_funds_of_non_existing_user() {
        let mut storage = UsersMemoryStorage::default();
        assert!(storage.show_funds("user1").is_err());
    }
}
