use anyhow::{Result, anyhow};
use trading_types::{OrderId, OrderToPlace};

use crate::types::traits::OrderPlacer;
use crate::types::values::Cred;

#[async_trait::async_trait]
impl OrderPlacer for super::Exmo {
    async fn place_order(&self, _cred: &Cred, _o: &OrderToPlace) -> Result<OrderId> {
        Err(anyhow!("can't place order"))
    }
}
