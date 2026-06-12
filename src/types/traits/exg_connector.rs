use anyhow::Result;
use async_trait::async_trait;

use trading_types::{Depth, OrderId, OrderPlaced, OrderToPlace, Symbol};

use crate::types::values::Cred;

#[async_trait]
pub trait ExgConnector: DepthFetcher + MyOrdersFetcher + OrderPlacer + OrderCanceler + ExgWsSubscriber {}

#[async_trait]
pub trait ExgWsSubscriber: Send {
    async fn subscribe_ws_public(&self, symbol: &Symbol) -> Result<()>;
    async fn subscribe_ws_private(&self, cred: &Cred) -> Result<()>;
}

#[async_trait]
pub trait DepthFetcher {
    async fn fetch_depth(&self, symbol: &Symbol) -> Result<Depth>;
}

#[async_trait]
pub trait OrderPlacer {
    async fn place_order(&self, cred: &Cred, o: &OrderToPlace) -> Result<OrderId>;
}

#[async_trait]
pub trait OrderCanceler {
    async fn cancel_order(&self, cred: &Cred, order_id: &OrderId) -> Result<bool>;
}

#[async_trait]
pub trait MyOrdersFetcher {
    async fn fetch_my_orders(&self) -> Result<Vec<OrderPlaced>>;
}
