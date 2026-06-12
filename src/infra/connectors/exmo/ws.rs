use super::Exmo;
use crate::{actors::ws_datastore_actor, types::traits::ExgWsSubscriber, types::values::Cred, ws_utils};
use anyhow::{Context, Result};
use async_trait::async_trait;
use bytes::Bytes;
use serde::Deserialize;
use trading_types::{Depth, Symbol};

#[async_trait]
impl ExgWsSubscriber for Exmo {
    async fn subscribe_ws_private(&self, _cred: &Cred) -> Result<()> {
        log::info!("exmo: no priv ws subscribtions");
        Ok(())
    }
    async fn subscribe_ws_public(&self, s: &Symbol) -> Result<()> {
        let ctx = format!("exmo: subscribe_ws_public: {s}");
        let _ = ws_datastore_actor::actor_ref(&ws_datastore_actor::datastore_actor_name(s))?;
        log::info!("{ctx}");
        let topic = super::symbol_to_depth_subscribe_topic(s);
        let topic_url = super::depth_subscribe_topic_url(s);
        let on_connect_send = serde_json::json!({
            "id": 1,
            "method": "subscribe",
            "topics": [topic]
        })
        .to_string();
        tokio::spawn(async move {
            ws_utils::ws_subscribe(topic_url, Some(&on_connect_send), process_ws_message_data).await;
        });
        Ok(())
    }
}

// https://documenter.getpostman.com/view/10287440/SzYXWKPi#308cf5d9-773b-48c0-8e0d-5a1c2ca40751
// Одна функция для разных вариантов данных
// Надо на случай если несолько топиков обрабатываются (что вполне возможно и даже )
#[cfg_attr(feature = "hotpath", hotpath::measure)]
async fn process_ws_message_data(msg_data: Bytes) -> Result<()> {
    use ws_datastore_actor::{depth_add_diff, depth_set_snapshot};
    #[derive(Debug, Deserialize)]
    struct DepthStreamed {
        // ts: i64,
        event: String, // snapshot / update
        topic: String, // spot/order_book_snapshots:ETH_USDT
        data: DepthData,
    }

    #[derive(Debug, Deserialize)]
    struct DepthData {
        ask: Vec<Vec<String>>,
        bid: Vec<Vec<String>>,
    }

    let ctx = "exmo: process_ws_message_data";
    if let Ok(depth_streamed) = serde_json::from_slice::<DepthStreamed>(&msg_data) {
        log::debug!("{ctx}: depth streamed: {:?}", &depth_streamed);
        let symbol = super::symbol_from_topic(&depth_streamed.topic).context(ctx)?;
        let depth = Depth::from((depth_streamed.data.ask, depth_streamed.data.bid));
        log::debug!("{ctx}: received depth {:?}", depth);
        let ws_datastore_actor_name = ws_datastore_actor::datastore_actor_name(&symbol);
        match depth_streamed.event.as_str() {
            "update" => depth_add_diff(&ws_datastore_actor_name, depth).await,
            "snapshot" => depth_set_snapshot(&ws_datastore_actor_name, depth).await,
            ev => anyhow::bail!("bad event: {}", ev),
        }
        .context(ctx)?;
    } else {
        // {"success":true,"ret_msg":"","conn_id":"e7482691-a775-49b1-8d9f-c670504d238f","req_id":"","op":"subscribe"}
        log::warn!("{ctx}: unknown {:?}", msg_data);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ws() {
        let exg = Exmo::new();
        exg.subscribe_ws_public(&Symbol::try_from("exmo:BTC/USDT").unwrap())
            .await
            .unwrap();
    }
}
