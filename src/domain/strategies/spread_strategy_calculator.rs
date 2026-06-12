use anyhow::Result;

use crate::types::{
    traits::StrategyCalculator,
    values::{SpreadExgData, StrategyCalc},
};

pub struct SpreadStrategyCalculator;

impl StrategyCalculator<SpreadExgData> for SpreadStrategyCalculator {
    fn calc(&self, _exg_data: &SpreadExgData) -> Result<StrategyCalc> {
        // TODO IMPL
        Ok(StrategyCalc {
            orders_to_place: Vec::new(),
            order_ids_to_cancel: Vec::new(),
            meta: serde_json::json!("{}"),
        })
    }
}
