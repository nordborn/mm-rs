use super::OkxWs;
use crate::types::traits::OrderCanceler;
use anyhow::Result;
use trading_types::OrderId;

#[async_trait::async_trait]
impl OrderCanceler for OkxWs {
    async fn cancel_order(&self, order_id: &OrderId) -> Result<bool> {
        self.okx_connector.cancel_order(order_id).await
    }
}
