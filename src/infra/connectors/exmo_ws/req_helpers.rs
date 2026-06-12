use super::ExmoWs;
use anyhow::{Context, Result};
use trading_types::Symbol;
use url::Url;

use crate::infra::connectors::Exmo;

impl ExmoWs {
    pub fn depth_subscribe_topic_url(s: &Symbol) -> Url {
        let topic = ExmoWs::symbol_to_depth_subscribe_topic(s);
        Url::parse(&format!("wss://ws-api.exmo.com:443/v1/public/{topic}")).unwrap()
    }

    pub fn symbol_to_depth_subscribe_topic(s: &Symbol) -> String {
        format!("spot/order_book_snapshots:{}", Exmo::symbol_to_pair(s))
    }

    // parses spot/order_book_snapshots:ETH_USDT
    pub fn symbol_from_topic(topic: &str) -> Result<Symbol> {
        let ctx = format!("symbol_from_topic: {topic}");
        let pair = topic.split(":").last().with_context(|| ctx.clone())?;
        Exmo::pair_to_symbol(pair).with_context(|| ctx)
    }
}
