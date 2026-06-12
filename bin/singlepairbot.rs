use std::time::Duration;

use anyhow::{Context, Result};

use clap::Parser;
use itertools::Itertools;
use mm::{
    app::bots::SinglePairBot,
    domain::strategies::SpreadStrategyCalculator,
    infra::{
        actors::{
            bot_config_provider_actor::BotConfigProviderActorAdapter,
            kvstore_actor::KVStoreActorAdapter,
            ws_market_data_provider_actor::{WsMarketDataProviderActorAdapter, WsParseFn},
        },
        config_providers::{BotConfigProviderDB, BotConfigProviderFs},
        connectors::{Exmo, ExmoWs, Okx, OkxWs},
        stores::{DefaultWsMarketDataStore, RedisStore},
    },
    types::{
        traits::{BotConfigProvider, ExgConnector, KVStore},
        values::{Cred, SvcConfig},
    },
};
use trading_types::Symbol;

const REFRESH_CONFIGS_EVERY: Duration = Duration::from_secs(5);
const RECONNECT_REDIS_EVERY: Duration = Duration::from_secs(5);

#[tokio::main]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [99]))]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    env_logger::init();
    let config = SvcConfig::parse();
    dbg!(&config.configs_dir, &config.creds_dir);
    log::info!("START SINGLE BOT {}", &config.bot_name);

    let config_provider_actor = BotConfigProviderActorAdapter::try_new(
        "config_provider:default".into(),
        get_config_provider(&config)?,
        REFRESH_CONFIGS_EVERY,
    )
    .await?;

    let external_kvstore_actor = KVStoreActorAdapter::try_new(
        "external_kvstore_actor:default",
        &config.redis_store_url,
        RECONNECT_REDIS_EVERY,
        get_external_kvstore(&config)?,
    )
    .await?;

    let exg_name = config.bot_name.split("_").collect_vec()[0];

    let ws_market_data_provider = WsMarketDataProviderActorAdapter::try_new(
        "ws_market_data_provider",
        Duration::from_secs(5),
        Duration::from_secs(10),
        Box::new(DefaultWsMarketDataStore::new(
            "market_datastore",
            Box::new(external_kvstore_actor.clone()),
        )),
        get_ws_parse_fn(exg_name),
    )
    .await?;

    let creds = get_creds(&config.bot_name);
    let symbol = Symbol::try_from(":/")?; // will set actual on bot try_new

    tokio::time::sleep(Duration::from_secs(1)).await;
    let mut bot = SinglePairBot::try_new(
        &config.bot_name,
        Box::new(SpreadStrategyCalculator),
        get_exg_connector(exg_name, creds, symbol, ws_market_data_provider),
        Box::new(config_provider_actor),
        Box::new(external_kvstore_actor),
    )
    .await?;

    bot.trade_loop().await
}

fn get_external_kvstore(config: &SvcConfig) -> Result<Box<dyn KVStore>> {
    let ctx = "run_redis_store_actor";
    let redis_store = RedisStore::try_new(&config.redis_store_url).context(ctx)?;
    Ok(Box::new(redis_store))
}

fn get_config_provider(config: &SvcConfig) -> Result<Box<dyn BotConfigProvider>> {
    let ctx = "get_config_provider";
    let provider: Box<dyn BotConfigProvider> = if !config.configs_dir.is_empty() {
        Box::new(BotConfigProviderFs::new(&config.configs_dir))
    } else {
        Box::new(BotConfigProviderDB::try_new(&config.configs_url).context(ctx)?)
    };
    Ok(provider)
}

fn get_creds(_bot_name: &str) -> Cred {
    // TODO: impl
    Cred::default()
}

fn get_ws_parse_fn(exg_name: &str) -> Box<WsParseFn> {
    match exg_name {
        "okx" => Box::new(OkxWs::ws_parse_fn),
        "exmo" => Box::new(ExmoWs::ws_parse_fn),
        _ => panic!("unknown exchange"),
    }
}

fn get_exg_connector(
    exg_name: &str,
    creds: Cred,
    symbol: Symbol,
    ws_market_data_provider: WsMarketDataProviderActorAdapter,
) -> Box<dyn ExgConnector> {
    match exg_name {
        "okx" => Box::new(OkxWs::new(Okx::new(creds, symbol), ws_market_data_provider)),
        "exmo" => Box::new(ExmoWs::new(Exmo::new(creds, symbol), ws_market_data_provider)),
        _ => panic!("unknown exchange"),
    }
}
