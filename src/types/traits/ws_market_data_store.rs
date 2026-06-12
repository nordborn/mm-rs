use anyhow::Result;
use async_trait::async_trait;
use trading_types::Depth;

#[async_trait]
pub trait WsMarketDataStore: Send + Sync + 'static {
    fn name(&self) -> &str;
    fn depth_get(&mut self) -> Result<Box<Depth>>;
    fn depth_set_snapshot(&mut self, depth: Depth);
    fn depth_add_diff(&mut self, depth: Depth);
    fn depth_merge_diffs(&mut self);
    async fn export_to_external_kvstore(&mut self) -> Result<()>;
}
