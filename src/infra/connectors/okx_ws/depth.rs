use super::OkxWs;
use crate::types::traits::DepthFetcher;
use anyhow::Result;
use async_trait::async_trait;
use trading_types::Depth;

#[async_trait]
impl DepthFetcher for OkxWs {
    /// gets it from ws data provider, no real fetch needed
    async fn fetch_depth(&mut self) -> Result<Box<Depth>> {
        self.ws_market_data_provider.depth_get().await
    }
}
