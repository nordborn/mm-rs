use super::EnhConfig;
use serde::Deserialize;
use trading_types::{Price, Side, Symbol, Worth};

#[derive(Debug, Clone)]
pub enum BotConfig {
    Multi(MultiBotConfig),
    Single(SingleBotConfig),
}

#[derive(Debug, Deserialize, Clone)]
pub struct MultiBotConfig {
    pub name: String,
    pub max_bots_in_deals: i32,
    pub bot_names: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SingleBotConfig {
    pub trade_iter_interval_millis: u64,
    pub symbol: Symbol,
    pub bs_precision: i8,
    pub qt_precision: i8,
    pub step: Price,
    pub min_order: Worth,
    pub max_in_deals: Worth,
    pub prefer_side: Side,
    pub min_spread_pct: f64,
    pub fee_pct: f64,
    pub enh: EnhConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let s = r#"
            {
                "symbol": {"eg": "exmo", "bs": "BTC", "qt": "USDT"},
                "bs_precision": 6,
                "qt_precision": 2,
                "step": 0.0001,
                "min_order": 1,
                "max_in_deals": 1000,
                "prefer_side": "buy",
                "min_spread_pct": 0.2,
                "enh": {
                    "depth_offsetter": {
                        "drop_cumsum": 10
                    }
                }
            }
        "#;
        let _: SingleBotConfig = serde_json::from_str(s).unwrap();
    }
}
