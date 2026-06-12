use anyhow::{Result, anyhow};
use trading_types::OrderId;

use crate::types::{traits::OrderCanceler, values::Cred};

#[async_trait::async_trait]
impl OrderCanceler for super::Exmo {
    async fn cancel_order(&self, _cred: &Cred, _order_id: &OrderId) -> Result<bool> {
        Err(anyhow!("can't cancel order"))
    }
}
