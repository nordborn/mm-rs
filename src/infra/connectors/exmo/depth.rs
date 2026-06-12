use super::Exmo;
use crate::types::traits::DepthFetcher;
use anyhow::Result;
use async_trait::async_trait;
use trading_types::Depth;

#[async_trait]
impl DepthFetcher for Exmo {
    async fn fetch_depth(&mut self) -> Result<Box<Depth>> {
        todo!()
    }
}
