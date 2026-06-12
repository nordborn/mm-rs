use super::WsParseFn;
use crate::infra::actors::ws_market_data_provider_actor::WsSubscribeParams;
use crate::types::traits::WsMarketDataStore;
use crate::types::values::WsMarketData;
use anyhow::Result;
use futures_util::{
    SinkExt,   // write.send()
    StreamExt, // stream.split()
};
use kameo::Actor;
use kameo::message::Message;
use tokio::time::Duration;
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
use trading_types::Depth;

#[derive(Actor)]
pub struct WsMarketDataProviderActor {
    ws_market_data_store: Box<dyn WsMarketDataStore>,
    ws_parse_fn: Box<WsParseFn>,
}

pub mod msg {

    use super::*;

    pub struct DepthGet;

    pub struct DepthMergeDiffsEvery(pub Duration);
    pub struct ExportToKVStoreEvery(pub Duration);
    pub struct Subscribe(pub WsSubscribeParams);
    pub struct OnWsMsg(pub WsMessage);
}

impl WsMarketDataProviderActor {
    pub fn new(ws_market_data_store: Box<dyn WsMarketDataStore>, ws_parse_fn: Box<WsParseFn>) -> Self {
        Self {
            ws_market_data_store,
            ws_parse_fn,
        }
    }
}

impl Message<msg::DepthGet> for WsMarketDataProviderActor {
    type Reply = Result<Box<Depth>>;

    async fn handle(
        &mut self,
        _msg: msg::DepthGet,
        _ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        log::debug!("got msg DepthGet: {}", self.ws_market_data_store.name());
        self.ws_market_data_store.depth_get()
    }
}

#[cfg_attr(feature = "hotpath", hotpath::measure_all)]
impl Message<msg::DepthMergeDiffsEvery> for WsMarketDataProviderActor {
    type Reply = ();

    async fn handle(
        &mut self,
        msg::DepthMergeDiffsEvery(dur): msg::DepthMergeDiffsEvery,
        ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        log::debug!("got msg DepthMergeDiffsEvery: {}", self.ws_market_data_store.name());
        self.ws_market_data_store.depth_merge_diffs();
        let actor_ref = ctx.actor_ref().clone();
        tokio::spawn(async move {
            tokio::time::sleep(dur).await;
            let _ = actor_ref.tell(msg::DepthMergeDiffsEvery(dur)).await;
        });
    }
}

impl Message<msg::ExportToKVStoreEvery> for WsMarketDataProviderActor {
    type Reply = ();

    async fn handle(
        &mut self,
        msg::ExportToKVStoreEvery(dur): msg::ExportToKVStoreEvery,
        ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        log::debug!("got msg ExportToRedisEvery: {}", self.ws_market_data_store.name());

        let _ = self
            .ws_market_data_store
            .export_to_external_kvstore()
            .await
            .map_err(|e| log::warn!("ws_datastore: ExportToRedisEvery: {e:#?}"));

        let actor_ref = ctx.actor_ref().clone();
        tokio::spawn(async move {
            tokio::time::sleep(dur).await;
            let _ = actor_ref.tell(msg::ExportToKVStoreEvery(dur)).await;
        });
    }
}

impl Message<msg::Subscribe> for WsMarketDataProviderActor {
    type Reply = ();

    async fn handle(
        &mut self,
        msg::Subscribe(subscribe_params): msg::Subscribe,
        ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let topic_url = subscribe_params.topic_url.clone();
        let on_connect_send = subscribe_params.on_connect_send.clone();
        let fn_ctx = format!("{topic_url}: ws");
        log::info!("{fn_ctx}: >> SUBSCRIBE <<");

        let fn_ctx_outer = fn_ctx.clone();
        let actor_ref = ctx.actor_ref().clone();
        tokio::spawn(async move {
            match connect_async(topic_url.as_str()).await {
                Ok((stream, _)) => {
                    log::info!("{fn_ctx_outer}: connected");
                    let (mut write, mut read) = stream.split();
                    #[cfg(feature = "hotpath")]
                    let mut read = hotpath::stream!(read, log = true, label = topic_url);

                    if let Some(txt) = on_connect_send {
                        let _ = write.send(WsMessage::Text(txt.into())).await;
                    }

                    while let Some(msg) = read.next().await {
                        match msg {
                            Err(e) => {
                                log::error!("{fn_ctx_outer}: err msg: {e:#?}");
                                // consider reconnect here
                            }
                            Ok(WsMessage::Ping(data)) => {
                                let _ = write.send(WsMessage::Pong(data)).await;
                            }
                            Ok(msg) => {
                                // IMPORTANT POINT: to mutate the state we will
                                // process the data as the actor message
                                let _ = actor_ref.tell(msg::OnWsMsg(msg)).await;
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("{fn_ctx_outer}: error connecting to server: {:#?}, repeat", e);
                }
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
            let _ = actor_ref.tell(msg::Subscribe(subscribe_params)).await;
        });
        log::info!("{fn_ctx}: >>> SUBSCRIBED <<<")
    }
}

impl Message<msg::OnWsMsg> for WsMarketDataProviderActor {
    type Reply = ();
    async fn handle(
        &mut self,
        msg::OnWsMsg(msg): msg::OnWsMsg,
        _ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        use WsMarketData::*;
        let fn_ctx = "ws msg";
        log::info!("{fn_ctx}: {:#}", &msg);
        match (*self.ws_parse_fn)(msg.into_data()) {
            Err(e) => log::warn!("{fn_ctx}: ws_parse: {e:#?}"),
            Ok(market_data) => match market_data {
                Unknown(data) => log::warn!("{fn_ctx}: unkwown data: {data}"),
                DepthDiff(data) => {
                    log::debug!("depth diff received");
                    self.ws_market_data_store.depth_add_diff(data)
                }
                DepthSnapshot(data) => {
                    log::debug!("depth snapshot received");
                    self.ws_market_data_store.depth_set_snapshot(data)
                }
                any => log::warn!("{fn_ctx}: not implemented: {any:?}"),
            },
        }
    }
}
