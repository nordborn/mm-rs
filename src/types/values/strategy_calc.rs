use serde::{Deserialize, Serialize};
use trading_types::OrderToPlace;

#[derive(Debug, Serialize, Deserialize)]
pub struct StrategyCalc {
    pub orders_to_place: Vec<OrderToPlace>,
    pub order_ids_to_cancel: Vec<String>,
    pub meta: serde_json::Value,
}
