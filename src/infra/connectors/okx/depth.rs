use super::Okx;
use crate::types::traits::DepthFetcher;
use anyhow::Result;
use async_trait::async_trait;
use trading_types::Depth;

#[async_trait]
impl DepthFetcher for Okx {
    async fn fetch_depth(&mut self) -> Result<Box<Depth>> {
        todo!()
    }
}
