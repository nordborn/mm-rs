use super::OkxWs;
use anyhow::Result;
use bytes::Bytes;
use chrono::Utc;
use serde::Deserialize;
use trading_types::Depth;

use crate::{
    infra::{actors::ws_market_data_provider_actor::WsSubscribeParams, connectors::Okx},
    types::values::WsMarketData,
};

#[derive(Debug, Deserialize)]
struct DepthStreamed {
    // ts: i64,
    // channel: String,
    action: String, // snapshot / update
    // topic: String, // spot/order_book_snapshots:ETH_USDT
    data: Vec<DepthData>,
}

#[derive(Debug, Deserialize)]
struct DepthData {
    asks: Vec<Vec<String>>,
    bids: Vec<Vec<String>>,
}

impl OkxWs {
    pub async fn on_init_subscribe_ws_private(&self) -> Result<()> {
        log::info!("okx: no priv ws subscribtions");
        Ok(())
    }
    pub async fn on_init_subscribe_ws_public(&self) -> Result<()> {
        let s = self.okx_connector.symbol.clone();
        let ctx = format!("okx: subscribe_ws_public: {s}");
        log::info!("{ctx}");
        let ws_subscribe_params = WsSubscribeParams {
            topic_url: Self::public_subscribe_url(),
            on_connect_send: Some(
                serde_json::json!({
                    "id": format!("{}", Utc::now().timestamp_millis()),
                    "op": "subscribe",
                    "args": [{
                        "channel": "books",
                        "instId": Okx::symbol_to_pair(&s)
                    }]
                })
                .to_string(),
            ),
        };

        self.ws_market_data_provider.subscribe(ws_subscribe_params).await
    }

    pub fn ws_parse_fn(msg_data: Bytes) -> Result<WsMarketData> {
        use WsMarketData::*;
        let ctx = "okx: ws_stream_parse";
        if let Ok(mut depth_streamed) = serde_json::from_slice::<DepthStreamed>(&msg_data) {
            log::debug!("{ctx}: depth streamed: {:?}", &depth_streamed);
            let data = depth_streamed.data.remove(0);
            let depth = Depth::from((data.asks, data.bids));
            Ok(match depth_streamed.action.as_str() {
                "update" => DepthDiff(depth),
                "snapshot" => DepthSnapshot(depth),
                _ev => Unknown(format!("{:?}", msg_data)),
            })
        } else {
            // {"success":true,"ret_msg":"","conn_id":"e7482691-a775-49b1-8d9f-c670504d238f","req_id":"","op":"subscribe"}
            Ok(Unknown(format!("{:?}", msg_data)))
        }
    }
}
