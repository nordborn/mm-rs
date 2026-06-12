use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct EnhConfig {
    pub depth_offsetter: DepthOffsetterConfig,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct DepthOffsetterConfig {
    pub drop_cumsum: trading_types::Amount,
}
