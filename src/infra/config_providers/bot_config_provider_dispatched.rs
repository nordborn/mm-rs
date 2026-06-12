/// This approach demonstrates enum-based polymorhism
/// and can be used intead of traits.
/// This is generally suitable for cases with different exposed behavior,
/// not used in the project for now.
use super::{BotConfigProviderDB, BotConfigProviderFs};
use crate::types::{
    traits::BotConfigProvider,
    values::{BotConfig, BotName},
};
use anyhow::{Context, Result};

pub enum BotConfigProviderDispather {
    Fs(BotConfigProviderFs),
    DB(BotConfigProviderDB),
}

pub struct BotConfigProviderDispatched {
    dispatcher: BotConfigProviderDispather,
}

impl BotConfigProviderDispatched {
    pub fn new(dispatcher: BotConfigProviderDispather) -> Self {
        Self { dispatcher }
    }

    pub async fn read_configs(&mut self) -> Result<()> {
        use BotConfigProviderDispather::*;
        match &mut self.dispatcher {
            DB(_) => todo!(),
            Fs(bot_conf_provider_fs) => bot_conf_provider_fs.read_configs().await,
        }
        .context("read_configs")
    }

    pub async fn get_config(&mut self, bot_name: &BotName) -> Result<Option<BotConfig>> {
        use BotConfigProviderDispather::*;
        match &mut self.dispatcher {
            DB(_) => todo!(),
            Fs(bot_conf_provider_fs) => bot_conf_provider_fs.get_config(bot_name).await,
        }
    }
}
