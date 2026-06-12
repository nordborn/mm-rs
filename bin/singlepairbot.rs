use std::time::Duration;

use anyhow::{Context, Result};

use clap::Parser;
use mm::{
    actors::{bot_config_provider_actor, redis_store_actor},
    app::bots::SinglePairBot,
    domain::strategies::DefaultSpreadStrategy,
    infra::{
        config_providers::{BotConfigProviderDB, BotConfigProviderFs},
        connectors::Exmo,
        stores::RedisStore,
    },
    types::{
        traits::BotConfigProvider,
        values::{Cred, SvcConfig},
    },
};

#[tokio::main]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [99]))]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    env_logger::init();
    let config = SvcConfig::parse();
    dbg!(&config.configs_dir, &config.creds_dir);
    log::info!("START SINGLE BOT {}", &config.bot_name);
    run_redis_store_actor(&config).await?;
    run_bot_config_provider_actor(&config).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let mut bot = SinglePairBot::try_new(&config.bot_name, Cred::default(), DefaultSpreadStrategy, Exmo).await?;
    bot.init().await?;
    bot.trade_loop().await
}

async fn run_redis_store_actor(config: &SvcConfig) -> Result<()> {
    let ctx = "run_redis_store_actor";
    let redis_store = RedisStore::try_new(&config.redis_store_url).context(ctx)?;
    redis_store_actor::spawn_register(redis_store_actor::REDIS_STORE_DEFAULT_NAME, redis_store)
        .await
        .context(ctx)?;
    // use actor ref
    redis_store_actor::reconnect_every(redis_store_actor::REDIS_STORE_DEFAULT_NAME, Duration::from_secs(5))
        .await
        .context(ctx)?;
    Ok(())
}

async fn run_bot_config_provider_actor(config: &SvcConfig) -> Result<()> {
    let ctx = "run_bot_config_provider_actor";
    let provider: Box<dyn BotConfigProvider> = if !config.configs_dir.is_empty() {
        Box::new(BotConfigProviderFs::new(&config.configs_dir))
    } else {
        Box::new(BotConfigProviderDB::new(&config.configs_url))
    };
    let name = bot_config_provider_actor::BOT_CONFIG_PROVIDER_DEFAULT_NAME;
    // actor messaging using its facade
    bot_config_provider_actor::spawn_register(name, provider)
        .await
        .context(ctx)?;
    // use actor name
    bot_config_provider_actor::refresh_configs_every(name, Duration::from_secs(5))
        .await
        .context(ctx)?;

    Ok(())
}
