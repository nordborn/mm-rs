use crate::types::values::{SpreadExgData, StrategyCalc};
use anyhow::Result;

pub trait SpreadStrategist {
    fn calc(&self, exg_data: &SpreadExgData) -> Result<StrategyCalc>;
}
