use super::ExmoWs;
use crate::types::traits::OrderPlacer;
use anyhow::Result;
use trading_types::{OrderId, OrderToPlace};

#[async_trait::async_trait]
impl OrderPlacer for ExmoWs {
    async fn place_order(&self, o: &OrderToPlace) -> Result<OrderId> {
        self.exmo_connector.place_order(o).await
    }
}
