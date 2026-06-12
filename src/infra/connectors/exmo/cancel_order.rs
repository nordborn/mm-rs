use super::Exmo;
use crate::types::traits::OrderCanceler;
use anyhow::{Result, anyhow};
use trading_types::OrderId;

#[async_trait::async_trait]
impl OrderCanceler for Exmo {
    async fn cancel_order(&self, _order_id: &OrderId) -> Result<bool> {
        Err(anyhow!("can't cancel order"))
    }
}
