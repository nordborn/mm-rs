use super::ExmoWs;
use crate::types::traits::MyOrdersFetcher;
use anyhow::Result;
use async_trait::async_trait;
use trading_types::OrderPlaced;

#[async_trait]
impl MyOrdersFetcher for ExmoWs {
    async fn fetch_my_orders(&self) -> Result<Vec<OrderPlaced>> {
        self.exmo_connector.fetch_my_orders().await
    }
}
