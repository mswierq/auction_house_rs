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

pub struct BackendService<UBT, ABT>
where
    UBT: UsersBackend + Send + 'static,
    ABT: AuctionsBackend + Send + 'static,
{
    users: Arc<Mutex<UBT>>,
    auctions: Arc<Mutex<ABT>>,
}

impl<UBT, ABT> Default for BackendService<UBT, ABT>
where
    UBT: UsersBackend + Default + Send + 'static,
    ABT: AuctionsBackend + Default + Send + 'static,
{
    fn default() -> Self {
        Self {
            users: Arc::new(Mutex::new(UBT::default())),
            auctions: Arc::new(Mutex::new(ABT::default())),
        }
    }
}

pub type DefaultBackendService = BackendService<UsersMemoryStorage, AuctionsMemoryStorage>;

#[tonic::async_trait]
impl<UBT, ABT> Backend for BackendService<UBT, ABT>
where
    UBT: UsersBackend + Send + 'static,
    ABT: AuctionsBackend + Send + 'static,
{
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
