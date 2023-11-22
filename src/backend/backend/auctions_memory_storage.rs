use crate::backend::{Auction, Funds};
use std::collections::HashMap;
use std::error::Error;

type MemoryStorageAuctionId = u64;

#[derive(Default)]
pub struct AuctionsMemoryStorage {
    auctions: HashMap<MemoryStorageAuctionId, Auction>,
    next_id: MemoryStorageAuctionId,
}

impl super::AuctionsBackend for AuctionsMemoryStorage {
    type AuctionId = MemoryStorageAuctionId;

    fn add_auction(&mut self, auction: Auction) -> Result<Self::AuctionId, Box<dyn Error>> {
        let auction_id = self.next_id;
        self.next_id += 1;
        if self.auctions.contains_key(&auction_id) {
            return Err("Lack of free auction ids!".into());
        }
        self.auctions.insert(auction_id, auction);
        Ok(auction_id)
    }

    fn bid_auction(
        &mut self,
        auction_id: Self::AuctionId,
        bidder: &str,
        amount: Funds,
    ) -> Result<(), Box<dyn Error>> {
        if self.auctions.contains_key(&auction_id) {
            let auction = self.auctions.get_mut(&auction_id).unwrap();
            if auction.seller == bidder {
                return Err("Seller cannot bid on their own auction".into());
            }
            if auction.buyer == Some(bidder.to_owned()) {
                return Err("Bidder is already the highest bidder".into());
            }
            if auction.current_price >= amount {
                return Err("Bid amount is lower than the current price".into());
            }
            if auction.end_time < std::time::SystemTime::now() {
                return Err("Auction is already concluded".into());
            }
            auction.buyer = Some(bidder.to_owned());
            auction.current_price = amount;
            Ok(())
        } else {
            Err("Auction does not exist".into())
        }
    }

    fn close_auction(&mut self, auction_id: Self::AuctionId) -> Result<Auction, Box<dyn Error>> {
        if self.auctions.contains_key(&auction_id) {
            {
                let auction = self.auctions.get(&auction_id).unwrap();
                if auction.end_time < std::time::SystemTime::now() {
                    return Err("Auction is already concluded".into());
                }
            }
            Ok(self.auctions.remove(&auction_id).unwrap())
        } else {
            Err("Auction does not exist".into())
        }
    }

    fn list_ongoing_auctions(&self) -> Result<HashMap<Self::AuctionId, Auction>, Box<dyn Error>> {
        Ok(self
            .auctions
            .iter()
            .filter(|(_, auction)| auction.end_time > std::time::SystemTime::now())
            .map(|(auction_id, auction)| (*auction_id, auction.clone()))
            .collect())
    }

    fn pop_concluded_auctions(
        &mut self,
    ) -> Result<HashMap<Self::AuctionId, Auction>, Box<dyn Error>> {
        let concluded = self
            .auctions
            .iter()
            .filter(|(_, auction)| auction.end_time <= std::time::SystemTime::now())
            .map(|(auction_id, auction)| (*auction_id, auction.clone()))
            .collect();
        for (auction_id, _) in &concluded {
            self.auctions.remove(auction_id);
        }
        Ok(concluded)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::backend::AuctionsBackend;
    #[test]
    fn test_add_auction() {
        let mut storage = AuctionsMemoryStorage::default();
        let auction = Auction::new("item", 0, std::time::Duration::from_secs(100), "seller");
        let auction_id = storage.add_auction(auction.clone()).unwrap();
        assert_eq!(storage.auctions.get(&auction_id), Some(&auction));
    }

    #[test]
    fn test_bid_auction() {
        let mut storage = AuctionsMemoryStorage::default();
        let auction = Auction::new("item", 0, std::time::Duration::from_secs(100), "seller");
        let auction_id = storage.add_auction(auction.clone()).unwrap();
        assert!(storage.bid_auction(auction_id, "bidder", 1).is_ok());
        let stored_auction = storage.auctions.get(&auction_id).unwrap();
        assert_eq!(stored_auction.buyer, Some("bidder".to_string()));
        assert_eq!(stored_auction.current_price, 1);
    }

    #[test]
    fn test_bid_auction_with_lower_price() {
        let mut storage = AuctionsMemoryStorage::default();
        let auction = Auction::new("item", 1, std::time::Duration::from_secs(100), "seller");
        let auction_id = storage.add_auction(auction.clone()).unwrap();
        assert!(storage.bid_auction(auction_id, "bidder", 0).is_err());
        let stored_auction = storage.auctions.get(&auction_id).unwrap();
        assert_eq!(stored_auction.buyer, None);
        assert_eq!(stored_auction.current_price, 0);
    }

    #[test]
    fn test_bid_auction_by_two_bidders() {
        let mut storage = AuctionsMemoryStorage::default();
        let auction = Auction::new("item", 0, std::time::Duration::from_secs(100), "seller");
        let auction_id = storage.add_auction(auction.clone()).unwrap();
        assert!(storage.bid_auction(auction_id, "bidder1", 1).is_ok());
        assert!(storage.bid_auction(auction_id, "bidder2", 2).is_ok());
        let stored_auction = storage.auctions.get(&auction_id).unwrap();
        assert_eq!(stored_auction.buyer, Some("bidder2".to_string()));
        assert_eq!(stored_auction.current_price, 2);
    }

    #[test]
    fn test_bid_auction_by_seller() {
        let mut storage = AuctionsMemoryStorage::default();
        let auction = Auction::new("item", 0, std::time::Duration::from_secs(100), "seller");
        let auction_id = storage.add_auction(auction.clone()).unwrap();
        assert!(storage.bid_auction(auction_id, "seller", 1).is_err());
        let stored_auction = storage.auctions.get(&auction_id).unwrap();
        assert_eq!(stored_auction.buyer, None);
        assert_eq!(stored_auction.current_price, 0);
    }

    #[test]
    fn test_bid_auction_that_does_not_exist() {
        let mut storage = AuctionsMemoryStorage::default();
        assert!(storage.bid_auction(0, "bidder", 1).is_err());
    }

    #[test]
    fn test_bid_auction_that_is_concluded() {
        let mut storage = AuctionsMemoryStorage::default();
        let auction = Auction::new("item", 0, std::time::Duration::from_secs(0), "seller");
        let auction_id = storage.add_auction(auction.clone()).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert!(storage.bid_auction(auction_id, "bidder", 1).is_err());
    }

    #[test]
    fn test_bid_auction_by_the_same_bidder() {
        let mut storage = AuctionsMemoryStorage::default();
        let auction = Auction::new("item", 0, std::time::Duration::from_secs(100), "seller");
        let auction_id = storage.add_auction(auction.clone()).unwrap();
        assert!(storage.bid_auction(auction_id, "bidder", 1).is_ok());
        assert!(storage.bid_auction(auction_id, "bidder", 2).is_err());
        let stored_auction = storage.auctions.get(&auction_id).unwrap();
        assert_eq!(stored_auction.buyer, Some("bidder".to_string()));
        assert_eq!(stored_auction.current_price, 1);
    }

    #[test]
    fn test_bid_auction_by_two_bidders_with_the_same_price() {
        let mut storage = AuctionsMemoryStorage::default();
        let auction = Auction::new("item", 0, std::time::Duration::from_secs(100), "seller");
        let auction_id = storage.add_auction(auction.clone()).unwrap();
        assert!(storage.bid_auction(auction_id, "bidder1", 1).is_ok());
        assert!(storage.bid_auction(auction_id, "bidder2", 1).is_err());
        let stored_auction = storage.auctions.get(&auction_id).unwrap();
        assert_eq!(stored_auction.buyer, Some("bidder1".to_string()));
        assert_eq!(stored_auction.current_price, 1);
    }

    #[test]
    fn test_close_auction() {
        let mut storage = AuctionsMemoryStorage::default();
        let auction = Auction::new("item", 0, std::time::Duration::from_secs(100), "seller");
        let auction_id = storage.add_auction(auction.clone()).unwrap();
        let closed_auction = storage.close_auction(auction_id).unwrap();
        assert_eq!(closed_auction.seller, "seller");
        assert_eq!(closed_auction.buyer, None);
        assert_eq!(closed_auction.current_price, 0);
        assert_eq!(closed_auction.item, "item");
        assert!(storage.auctions.get(&auction_id).is_none());
    }

    #[test]
    fn test_close_auction_that_does_not_exist() {
        let mut storage = AuctionsMemoryStorage::default();
        assert!(storage.close_auction(0).is_err());
    }

    #[test]
    fn test_close_auction_that_is_already_concluded() {
        let mut storage = AuctionsMemoryStorage::default();
        let auction = Auction::new("item", 0, std::time::Duration::from_secs(0), "seller");
        let auction_id = storage.add_auction(auction.clone()).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert!(storage.close_auction(auction_id).is_err());
    }

    #[test]
    fn test_list_ongoing_auctions() {
        let mut storage = AuctionsMemoryStorage::default();
        let auction1 = Auction::new("item1", 0, std::time::Duration::from_secs(100), "seller1");
        let auction2 = Auction::new("item2", 0, std::time::Duration::from_secs(0), "seller2");
        let auction3 = Auction::new("item3", 0, std::time::Duration::from_secs(100), "seller3");
        let auction_id1 = storage.add_auction(auction1.clone()).unwrap();
        let auction_id2 = storage.add_auction(auction2.clone()).unwrap();
        let auction_id3 = storage.add_auction(auction3.clone()).unwrap();
        let ongoing_auctions = storage.list_ongoing_auctions().unwrap();
        assert_eq!(ongoing_auctions.len(), 2);
        assert_eq!(ongoing_auctions[&auction_id1], auction1);
        assert_eq!(ongoing_auctions[&auction_id3], auction3);
    }

    #[test]
    fn test_pop_concluded_auctions() {
        let mut storage = AuctionsMemoryStorage::default();
        let auction1 = Auction::new("item1", 0, std::time::Duration::from_secs(100), "seller1");
        let auction2 = Auction::new("item2", 0, std::time::Duration::from_secs(0), "seller2");
        let auction3 = Auction::new("item3", 0, std::time::Duration::from_secs(100), "seller3");
        let auction_id1 = storage.add_auction(auction1.clone()).unwrap();
        let auction_id2 = storage.add_auction(auction2.clone()).unwrap();
        let auction_id3 = storage.add_auction(auction3.clone()).unwrap();
        let concluded_auctions = storage.pop_concluded_auctions().unwrap();
        assert_eq!(concluded_auctions.len(), 1);
        assert_eq!(concluded_auctions[&auction_id2], auction2);
        assert!(storage.auctions.get(&auction_id1).is_some());
        assert!(storage.auctions.get(&auction_id3).is_some());
        assert_eq!(storage.auctions.len(), 2);
    }
}
