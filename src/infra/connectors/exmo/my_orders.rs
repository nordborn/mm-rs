use anyhow::{Result, anyhow};
use trading_types::OrderPlaced;

use crate::types::traits::MyOrdersFetcher;

#[async_trait::async_trait]
impl MyOrdersFetcher for super::Exmo {
    async fn fetch_my_orders(&self) -> Result<Vec<OrderPlaced>> {
        Err(anyhow!("no orders fetched"))
    }
}
