use anyhow::Result;

use crate::types::{
    traits::SpreadStrategist,
    values::{SpreadExgData, StrategyCalc},
};

pub struct DefaultSpreadStrategy;

impl SpreadStrategist for DefaultSpreadStrategy {
    fn calc(&self, _exg_data: &SpreadExgData) -> Result<StrategyCalc> {
        // TODO IMPL
        Ok(StrategyCalc {
            orders_to_place: Vec::new(),
            order_ids_to_cancel: Vec::new(),
            meta: serde_json::json!("{}"),
        })
    }
}
