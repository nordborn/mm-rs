use super::Okx;
use crate::types::traits::OrderPlacer;
use anyhow::{Result, anyhow};
use trading_types::{OrderId, OrderToPlace};

#[async_trait::async_trait]
impl OrderPlacer for Okx {
    async fn place_order(&self, _o: &OrderToPlace) -> Result<OrderId> {
        Err(anyhow!("can't place order"))
    }
}
