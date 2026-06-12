use super::OkxWs;
use crate::types::traits::OrderPlacer;
use anyhow::Result;
use trading_types::{OrderId, OrderToPlace};

#[async_trait::async_trait]
impl OrderPlacer for OkxWs {
    async fn place_order(&self, o: &OrderToPlace) -> Result<OrderId> {
        self.okx_connector.place_order(o).await
    }
}
