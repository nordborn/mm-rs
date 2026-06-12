use super::Exmo;
use crate::types::traits::MyOrdersFetcher;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use trading_types::OrderPlaced;

#[async_trait]
impl MyOrdersFetcher for Exmo {
    async fn fetch_my_orders(&self) -> Result<Vec<OrderPlaced>> {
        Err(anyhow!("no orders fetched"))
    }
}
