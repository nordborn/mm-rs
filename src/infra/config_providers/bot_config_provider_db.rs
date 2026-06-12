use anyhow::Result;
use async_trait::async_trait;

use crate::types::{
    traits::BotConfigProvider,
    values::{BotConfig, BotName},
};

pub struct BotConfigProviderDB {
    _pg_url: String,
}

impl BotConfigProviderDB {
    pub fn new(_pg_url: &str) -> Self {
        todo!("new");
    }
}

#[async_trait]
impl BotConfigProvider for BotConfigProviderDB {
    async fn read_configs(&mut self) -> Result<()> {
        todo!("read db configs");
    }

    async fn get_config(&mut self, _bot_name: &BotName) -> Option<BotConfig> {
        todo!("get_config db configs");
    }
}
