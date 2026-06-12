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
    pub fn try_new(_pg_url: &str) -> Result<Self> {
        todo!("new");
    }
}

#[async_trait]
impl BotConfigProvider for BotConfigProviderDB {
    async fn read_configs(&mut self) -> Result<()> {
        todo!("read db configs");
    }

    async fn get_config(&self, _bot_name: &BotName) -> Result<Option<BotConfig>> {
        todo!("get_config db configs");
    }
}
