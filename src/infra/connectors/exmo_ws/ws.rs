use super::ExmoWs;
use anyhow::Result;
use bytes::Bytes;
use serde::Deserialize;
use trading_types::Depth;

use crate::{infra::actors::ws_market_data_provider_actor::WsSubscribeParams, types::values::WsMarketData};

#[derive(Debug, Deserialize)]
struct DepthStreamed {
    // ts: i64,
    event: String, // snapshot / update
    // topic: String, // spot/order_book_snapshots:ETH_USDT
    data: DepthData,
}

#[derive(Debug, Deserialize)]
struct DepthData {
    ask: Vec<Vec<String>>,
    bid: Vec<Vec<String>>,
}

impl ExmoWs {
    pub async fn on_init_subscribe_ws_private(&self) -> Result<()> {
        log::info!("exmo: no priv ws subscribtions");
        Ok(())
    }
    pub async fn on_init_subscribe_ws_public(&self) -> Result<()> {
        let s = self.exmo_connector.symbol.clone();
        let ctx = format!("exmo: subscribe_ws_public: {s}");
        log::info!("{ctx}");
        let ws_subscribe_params = WsSubscribeParams {
            topic_url: Self::depth_subscribe_topic_url(&s),
            on_connect_send: Some(
                serde_json::json!({
                    "id": 1,
                    "method": "subscribe",
                    "topics": [Self::symbol_to_depth_subscribe_topic(&s)]
                })
                .to_string(),
            ),
        };
        self.ws_market_data_provider.subscribe(ws_subscribe_params).await
    }

    pub fn ws_parse_fn(msg_data: Bytes) -> Result<WsMarketData> {
        use WsMarketData::*;
        let ctx = "exmo: ws_stream_parse";
        if let Ok(depth_streamed) = serde_json::from_slice::<DepthStreamed>(&msg_data) {
            log::debug!("{ctx}: depth streamed: {:?}", &depth_streamed);
            let depth = Depth::from((depth_streamed.data.ask, depth_streamed.data.bid));
            Ok(match depth_streamed.event.as_str() {
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
