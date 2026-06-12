use clap::Parser;

use super::BotName;

#[derive(Parser, Debug)]
pub struct SvcConfig {
    #[clap(env = "BOT_CONFIGS_URL")]
    pub configs_url: String,

    #[clap(env = "BOT_CONFIGS_DIR")]
    pub configs_dir: String,

    #[clap(env = "BOT_CREDS_DIR")]
    pub creds_dir: String,

    #[clap(env = "BOT_REDIS_STORE_URL")]
    pub redis_store_url: String,

    #[clap(short = 'b', long = "bot")]
    pub bot_name: BotName,
}
