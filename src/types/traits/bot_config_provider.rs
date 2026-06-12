use anyhow::Result;
use async_trait::async_trait;

use crate::types::values::{BotConfig, BotName};

#[async_trait]
pub trait BotConfigProvider: Send + 'static {
    async fn read_configs(&mut self) -> Result<()>;
    async fn get_config(&mut self, bot_name: &BotName) -> Option<BotConfig>;
}
