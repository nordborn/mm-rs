use std::time::Duration;

/// Функции - фасад модуля, скрывающие то, что мы обращаемся к актору,
/// выглядит просто как обращение к синглтону по его имени
use kameo::actor::{ActorRef, Spawn};

use anyhow::{Context, Result};

use super::BotConfigProviderActor;
use crate::types::{
    traits::BotConfigProvider,
    values::{BotConfig, BotName},
};

pub const BOT_CONFIG_PROVIDER_DEFAULT_NAME: &str = "bot_config_provider_default";

pub async fn spawn_register(register_name: &str, provider: Box<dyn BotConfigProvider>) -> Result<()> {
    let ctx = format!("spawn_register: {register_name}");
    let act = BotConfigProviderActor::try_new(provider).with_context(|| ctx.clone())?;
    let addr = BotConfigProviderActor::spawn(act);
    addr.wait_for_startup().await;
    addr.register(register_name.to_string()).with_context(|| ctx.clone())?;
    Ok(())
}

pub async fn refresh_configs_every(registered_name: &str, dur: Duration) -> Result<()> {
    get_actor_ref(registered_name)
        .tell(super::msg::RefreshConfigsEvery(dur))
        .await
        .with_context(|| format!("refresh_configs_every: {registered_name}"))
}

pub async fn get_config(registered_name: &str, bot_name: &BotName) -> Result<Option<BotConfig>> {
    get_actor_ref(registered_name)
        .ask(super::msg::GetConfig {
            bot_name: bot_name.to_string(),
        })
        .await
        .with_context(|| format!("get_config: {registered_name}: {bot_name}"))
}

fn get_actor_ref(registered_name: &str) -> ActorRef<BotConfigProviderActor> {
    ActorRef::lookup(registered_name).unwrap().unwrap()
}
