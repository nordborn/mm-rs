use kameo::Actor;
use kameo::message::Message;

use anyhow::Result;

use crate::types::{
    traits::BotConfigProvider,
    values::{BotConfig, BotName},
};

pub mod msg {
    use std::time::Duration;

    use super::*;

    pub struct GetConfig {
        pub bot_name: BotName,
    }

    pub struct RefreshConfigsEvery(pub Duration);
}

// Актор чтения конфигов.
// После спавна ожидает сообщения RefreshConfigsEvery(dur)
// и запускает цикл рефреша конфигов по интервалу
#[derive(Actor)]
pub struct BotConfigProviderActor {
    pub provider: Box<dyn BotConfigProvider>,
}

impl BotConfigProviderActor {
    // try_new в данном случае излишне, но если бы мы выделляли ресурсы (пул бд),
    // то имело бы смысл сделать
    pub fn try_new(provider: Box<dyn BotConfigProvider>) -> Result<Self> {
        Ok(Self { provider })
    }
}

impl Message<msg::GetConfig> for BotConfigProviderActor {
    type Reply = Option<BotConfig>;
    async fn handle(
        &mut self,
        msg: msg::GetConfig,
        _ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        self.provider.get_config(&msg.bot_name).await
    }
}

impl Message<msg::RefreshConfigsEvery> for BotConfigProviderActor {
    type Reply = ();
    async fn handle(
        &mut self,
        msg::RefreshConfigsEvery(dur): msg::RefreshConfigsEvery,
        ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        log::debug!("got msg RefreshConfigsEvery");
        let _ = self
            .provider
            .read_configs()
            .await
            .map_err(|e| log::error!("can't read configs: {e}"));
        let actor_ref = ctx.actor_ref().clone();
        tokio::spawn(async move {
            tokio::time::sleep(dur).await;
            let _ = actor_ref.tell(msg::RefreshConfigsEvery(dur)).await;
        });
    }
}
