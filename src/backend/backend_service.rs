use backend_proto::backend_server::Backend;
use backend_proto::{
    BidItemRequest, DepositFundsRequest, DepositItemRequest, ListAuctionsResponse, SellItemRequest,
    ShowFundsResponse, ShowItemsResponse, WatchUserAuctionsResponse, WithdrawFundsRequest,
    WithdrawItemRequest,
};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
pub mod backend_proto {
    tonic::include_proto!("auction_house_rs.backend");
}

use crate::backend::{
    auctions_memory_storage::AuctionsMemoryStorage, users_memory_storage::UsersMemoryStorage,
    AuctionsBackend, UsersBackend,
};

type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;


pub struct BackendService {
    users: Arc<Mutex<dyn UsersBackend + Send>>,
    auctions: Arc<Mutex<dyn AuctionsBackend + Send>>,
}

impl Default for BackendService {
    fn default() -> Self {
        Self {
            users: Arc::new(Mutex::new(UsersMemoryStorage::default())),
            auctions: Arc::new(Mutex::new(AuctionsMemoryStorage::default())),
        }
    }
}

#[tonic::async_trait]
impl Backend for BackendService {
    async fn deposit_funds(
        &self,
        request: Request<DepositFundsRequest>,
    ) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn withdraw_funds(
        &self,
        request: Request<WithdrawFundsRequest>,
    ) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn show_funds(
        &self,
        request: Request<()>,
    ) -> Result<Response<ShowFundsResponse>, Status> {
        unimplemented!()
    }

    async fn deposit_item(
        &self,
        request: Request<DepositItemRequest>,
    ) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn withdraw_item(
        &self,
        request: Request<WithdrawItemRequest>,
    ) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn show_items(
        &self,
        request: Request<()>,
    ) -> Result<Response<ShowItemsResponse>, Status> {
        unimplemented!()
    }

    async fn sell_item(&self, request: Request<SellItemRequest>) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn bid_item(&self, request: Request<BidItemRequest>) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn list_auctions(
        &self,
        request: Request<()>,
    ) -> Result<Response<ListAuctionsResponse>, Status> {
        unimplemented!()
    }

    type WatchAuctionsStream = ResponseStream<ListAuctionsResponse>;
    async fn watch_auctions(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::WatchAuctionsStream>, Status> {
        unimplemented!()
    }

    type WatchUserAuctionsStream = ResponseStream<WatchUserAuctionsResponse>;

    async fn watch_user_auctions(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::WatchUserAuctionsStream>, Status> {
        unimplemented!()
    }
}
