use crate::backend::Funds;
use std::collections::{HashMap, HashSet};
use std::error::Error;

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
        todo!()
    }

    fn deposit_funds(&mut self, user: &str, amount: Funds) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn withdraw_funds(&mut self, user: &str, amount: Funds) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn deposit_item(&mut self, user: &str, item: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn withdraw_item(&mut self, user: &str, item: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn list_items(&self, user: &str) -> Result<Vec<String>, Box<dyn Error>> {
        todo!()
    }

    fn show_funds(&self, user: &str) -> Result<u32, Box<dyn Error>> {
        todo!()
    }
}

#[cfg(test)]
mod test {}
