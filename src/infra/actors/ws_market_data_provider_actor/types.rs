use anyhow::Result;
use bytes::Bytes;
use url::Url;

use crate::types::values::WsMarketData;

pub struct WsSubscribeParams {
    pub topic_url: Url,
    pub on_connect_send: Option<String>,
}

pub type WsParseFn = dyn Fn(Bytes) -> Result<WsMarketData> + Send + Sync;
