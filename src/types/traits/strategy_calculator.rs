use crate::types::values::StrategyCalc;
use anyhow::Result;

pub trait StrategyCalculator<T> {
    fn calc(&self, exg_data: &T) -> Result<StrategyCalc>;
}
