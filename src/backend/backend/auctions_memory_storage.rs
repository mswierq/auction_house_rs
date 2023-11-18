use crate::backend::{Auction, AuctionId, Funds};
use std::collections::HashMap;
use std::error::Error;

#[derive(Default)]
pub struct AuctionsMemoryStorage {
    auctions: HashMap<AuctionId, Auction>,
}

impl super::AuctionsBackend for AuctionsMemoryStorage {
    fn add_auction(&mut self, auction: Auction) -> Result<AuctionId, Box<dyn Error>> {
        todo!()
    }

    fn bid_auction(
        &mut self,
        auction_id: AuctionId,
        bidder: &str,
        amount: Funds,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn close_auction(&mut self, auction_id: AuctionId) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn list_ongoing_auctions(&self) -> Result<Vec<(AuctionId, Auction)>, Box<dyn Error>> {
        todo!()
    }

    fn watch_auctions(&self, user: &str) -> Result<Vec<(AuctionId, Auction)>, Box<dyn Error>> {
        todo!()
    }
}

#[cfg(test)]
mod test {}
