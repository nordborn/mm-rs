use super::OkxWs;
use crate::types::traits::MyOrdersFetcher;
use anyhow::Result;
use async_trait::async_trait;
use trading_types::OrderPlaced;

#[async_trait]
impl MyOrdersFetcher for OkxWs {
    async fn fetch_my_orders(&self) -> Result<Vec<OrderPlaced>> {
        self.okx_connector.fetch_my_orders().await
    }
}
