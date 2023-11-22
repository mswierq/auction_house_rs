use std::collections::HashMap;
pub mod auctions_memory_storage;
pub mod users_memory_storage;

type Funds = u32;

/// Trait for user data storage.
pub trait UsersBackend {
    /// Adds a new user with 0 funds and 0 items.
    ///
    /// # Arguments
    /// * `user` - The user's name.
    /// # Returns
    /// Should return an error if the user already exists.
    fn add_user(&mut self, user: &str) -> Result<(), Box<dyn std::error::Error>>;

    /// Deposits funds to the user's account.
    ///
    /// # Arguments
    /// * `user` - The user's name.
    /// * `amount` - The amount of funds to deposit.
    /// # Returns
    ///  Should return an error if the user does not exist or max funds exceeded.
    fn deposit_funds(
        &mut self,
        user: &str,
        amount: Funds,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Withdraws funds from the user's account.
    ///
    /// # Arguments
    /// * `user` - The user's name.
    /// * `amount` - The amount of funds to withdraw.
    /// # Returns
    /// Should return an error if the user does not exist or insufficient funds.
    fn withdraw_funds(
        &mut self,
        user: &str,
        amount: Funds,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Deposits an item to the user's account.
    ///
    /// # Arguments
    /// * `user` - The user's name.
    /// * `item` - The item's name.
    /// # Returns
    /// Should return an error if the user does not exist or item already exists.
    fn deposit_item(&mut self, user: &str, item: &str) -> Result<(), Box<dyn std::error::Error>>;

    /// Withdraws an item from the user's account.
    ///
    /// # Arguments
    /// * `user` - The user's name.
    /// * `item` - The item's name.
    /// # Returns
    /// Should return an error if the user does not exist or item does not exist.
    fn withdraw_item(&mut self, user: &str, item: &str) -> Result<(), Box<dyn std::error::Error>>;

    /// Lists the user's items.
    ///
    /// # Arguments
    /// * `user` - The user's name.
    /// # Returns
    /// Should return a vector of the user's items or an error if the user does not exist.
    fn list_items(&self, user: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;

    /// Show the user's funds.
    ///
    /// # Arguments
    /// * `user` - The user's name.
    /// # Returns
    /// Should return user's funds or an error if the user does not exist.
    fn show_funds(&self, user: &str) -> Result<u32, Box<dyn std::error::Error>>;
}

/// A struct representing an auction.
#[derive(Clone, PartialEq, Debug)]
pub struct Auction {
    item: String,
    starting_price: Funds,
    current_price: Funds,
    start_time: std::time::SystemTime,
    end_time: std::time::SystemTime,
    seller: String,
    buyer: Option<String>,
}

impl Auction {
    fn new(item: &str, starting_price: Funds, duration: std::time::Duration, seller: &str) -> Self {
        let start_time = std::time::SystemTime::now();
        let end_time = start_time + duration;
        Self {
            item: item.to_owned(),
            starting_price,
            current_price: 0,
            start_time,
            end_time,
            seller: seller.to_owned(),
            buyer: None,
        }
    }
}

/// Trait for auctions data storage.
pub trait AuctionsBackend {
    type AuctionId;

    /// Adds a new auction.
    ///
    /// # Arguments
    /// * `auction` - The auction to add.
    /// # Returns
    /// Should return an auction id or an error if adding the auction failed.
    fn add_auction(
        &mut self,
        auction: Auction,
    ) -> Result<Self::AuctionId, Box<dyn std::error::Error>>;

    /// Bids on an auction.
    ///
    /// # Arguments
    /// * `auction_id` - The auction's id.
    /// * `bidder` - The bidder's name.
    /// * `amount` - The amount of funds to bid.
    /// # Returns
    /// Should return an error if the auction does not exist, the auction is concluded, the bidder does not exist, the bidder is the seller, the bidder is the current highest bidder, or the bid amount is lower than the current price.
    fn bid_auction(
        &mut self,
        auction_id: Self::AuctionId,
        bidder: &str,
        amount: Funds,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Close an auction and return the auction's information.
    ///
    /// # Arguments
    /// * `auction_id` - The auction's id.
    /// # Returns
    /// Should return an auction or an error if the auction does not exist or the auction is already concluded.
    fn close_auction(
        &mut self,
        auction_id: Self::AuctionId,
    ) -> Result<Auction, Box<dyn std::error::Error>>;

    /// Lists all ongoing auctions.
    /// # Returns
    /// Should return a vector of all ongoing auctions with their ids or an error if listing the auctions failed.
    fn list_ongoing_auctions(
        &self,
    ) -> Result<HashMap<Self::AuctionId, Auction>, Box<dyn std::error::Error>>;

    /// Removes all concluded auctions from the storage and returns them.
    /// # Returns
    /// Should return a vector of all concluded auctions with their ids or an error if listing the auctions failed.
    fn pop_concluded_auctions(
        &mut self,
    ) -> Result<HashMap<Self::AuctionId, Auction>, Box<dyn std::error::Error>>;
}
