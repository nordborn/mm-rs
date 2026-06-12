use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait KVStore: Send + Sync + 'static {
    async fn get(&mut self, key: String) -> Result<String>;
    async fn set(&mut self, key: String, value: String) -> Result<()>;
    fn reconnect(&mut self) -> Result<()>;
}
