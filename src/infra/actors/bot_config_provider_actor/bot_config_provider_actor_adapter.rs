use std::time::Duration;

use anyhow::{Context, Result};
use async_trait::async_trait;
use kameo::actor::{ActorRef, Spawn};

use crate::{
    infra::actors::bot_config_provider_actor::BotConfigProviderActor,
    types::{
        traits::BotConfigProvider,
        values::{BotConfig, BotName},
    },
};

// we don't keep config provider here bcs it will be passed to the actor struct
pub struct BotConfigProviderActorAdapter {
    name: String,
    refresh_configs_every: Duration,
}

impl BotConfigProviderActorAdapter {
    pub async fn try_new(
        name: String,
        config_provider: Box<dyn BotConfigProvider>,
        refresh_configs_every: Duration,
    ) -> Result<Self> {
        let ctx = format!("BotConfigProviderActorAdapter: new: {name}");
        let obj = Self {
            name,
            refresh_configs_every,
        };
        obj.try_init(config_provider).await.with_context(|| ctx)?;
        Ok(obj)
    }

    // called from try_new
    async fn try_init(&self, config_provider: Box<dyn BotConfigProvider>) -> Result<()> {
        let ctx = "try_init";
        let name = &self.name;
        let act = BotConfigProviderActor::try_new(config_provider).context(ctx)?;
        let actor_ref = BotConfigProviderActor::spawn(act);
        actor_ref.wait_for_startup().await;
        actor_ref.register(name.clone()).context(ctx)?;
        self.on_init_refresh_configs_every().await.context(ctx)
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn actor_ref(&self) -> Result<ActorRef<BotConfigProviderActor>> {
        let name = self.name.as_str();
        ActorRef::lookup(name)
            .context("registry")?
            .with_context(|| format!("get_actor_ref: {name}"))
    }

    async fn on_init_refresh_configs_every(&self) -> Result<()> {
        let ctx = "refresh_configs_every";
        self.actor_ref()?
            .tell(super::msg::RefreshConfigsEvery(self.refresh_configs_every))
            .await
            .context(ctx)
    }
}

#[async_trait]
impl BotConfigProvider for BotConfigProviderActorAdapter {
    async fn read_configs(&mut self) -> Result<()> {
        // will be updated internally
        Ok(())
    }
    async fn get_config(&self, bot_name: &BotName) -> Result<Option<BotConfig>> {
        let name = &self.name;
        self.actor_ref()?
            .ask(super::msg::GetConfig {
                bot_name: bot_name.to_string(),
            })
            .await
            .with_context(|| format!("get_config: {name}: {bot_name}"))
    }
}
