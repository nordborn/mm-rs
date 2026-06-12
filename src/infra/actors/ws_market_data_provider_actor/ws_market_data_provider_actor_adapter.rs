use std::time::Duration;

use crate::infra::actors::ws_market_data_provider_actor::{WsMarketDataProviderActor, WsParseFn, WsSubscribeParams};
use anyhow::{Context, Result};
use kameo::actor::{ActorRef, Spawn};
use trading_types::Depth;

use crate::types::traits::WsMarketDataStore;

pub struct WsMarketDataProviderActorAdapter {
    pub name: String,
    pub merge_diffs_every: Duration,
    pub export_to_external_kvstore_every: Duration,
}

impl WsMarketDataProviderActorAdapter {
    pub async fn try_new(
        name: &str,
        merge_diffs_every: Duration,
        export_to_external_kvstore_every: Duration,
        ws_market_data_store: Box<dyn WsMarketDataStore>,
        ws_parse_fn: Box<WsParseFn>,
    ) -> Result<Self> {
        let ctx = format!("WsMarketDataProviderActorAdapter: try_new: {}", name);
        let obj = Self {
            name: name.into(),
            merge_diffs_every,
            export_to_external_kvstore_every,
        };
        obj.try_init(ws_market_data_store, ws_parse_fn)
            .await
            .with_context(|| ctx)?;
        Ok(obj)
    }

    // init with implementation-specific behavior
    async fn try_init(
        &self,
        ws_market_data_store: Box<dyn WsMarketDataStore>,
        ws_parse_fn: Box<WsParseFn>,
    ) -> Result<()> {
        let ctx = "try_init";
        let act = WsMarketDataProviderActor::new(ws_market_data_store, ws_parse_fn);
        let actor_ref = WsMarketDataProviderActor::spawn(act);
        actor_ref.wait_for_startup().await;
        actor_ref.register(self.name.clone()).context(ctx)?;
        self.on_init_merge_diffs_every(self.merge_diffs_every)
            .await
            .context(ctx)?;
        self.on_init_export_to_external_kvstore_every(self.export_to_external_kvstore_every)
            .await
            .context(ctx)?;
        Ok(())
    }

    fn actor_ref(&self) -> Result<ActorRef<WsMarketDataProviderActor>> {
        let name = self.name.as_str();
        ActorRef::lookup(name)
            .context("registry")?
            .with_context(|| format!("ws_datastore: get_actor_ref: {name}"))
    }

    async fn on_init_merge_diffs_every(&self, dur: Duration) -> Result<()> {
        self.actor_ref()?
            .tell(super::msg::DepthMergeDiffsEvery(dur))
            .await
            .context("merge_diffs_every")
    }

    async fn on_init_export_to_external_kvstore_every(&self, dur: Duration) -> Result<()> {
        self.actor_ref()?
            .tell(super::msg::ExportToKVStoreEvery(dur))
            .await
            .context("export_to_redis_every")
    }

    pub async fn subscribe(&self, params: WsSubscribeParams) -> Result<()> {
        self.actor_ref()?
            .tell(super::msg::Subscribe(params))
            .await
            .context("subscribe")
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn depth_get(&self) -> Result<Box<Depth>> {
        self.actor_ref()?.ask(super::msg::DepthGet).await.context("depth_get")
    }
}
