use anyhow::{Context, Result};
use trading_types::Symbol;
use url::Url;

// exmo:BTC/USD -> BTC_USD
pub fn symbol_to_pair(s: &Symbol) -> String {
    format!("{}_{}", s.bs.to_uppercase(), s.qt.to_uppercase())
}

// BTC_USD -> Symbol("exmo:BTC/USD")
pub fn pair_to_symbol(pair: &str) -> Result<Symbol> {
    let ctx = format!("pair_to_symbol: {pair}");
    let (b, q) = pair.split_once("_").with_context(|| ctx.clone())?;
    Ok(Symbol::new("exmo".into(), b.into(), q.into()))
}

pub fn depth_subscribe_topic_url(s: &Symbol) -> Url {
    let topic = symbol_to_depth_subscribe_topic(s);
    Url::parse(&format!("wss://ws-api.exmo.com:443/v1/public/{topic}")).unwrap()
}

pub fn symbol_to_depth_subscribe_topic(s: &Symbol) -> String {
    format!("spot/order_book_snapshots:{}", super::symbol_to_pair(s))
}

// parses spot/order_book_snapshots:ETH_USDT
pub fn symbol_from_topic(topic: &str) -> Result<Symbol> {
    let ctx = format!("symbol_from_topic: {topic}");
    let pair = topic.split(":").last().with_context(|| ctx.clone())?;
    super::pair_to_symbol(pair).with_context(|| ctx)
}
