use kameo::Actor;
use kameo::message::Message;

use crate::actors::redis_store_actor;
use trading_types::Depth;

use crate::infra::stores::WsDatastore;
use std::time::Duration;

#[derive(Actor)]
pub struct WsDatastoreActor {
    store: WsDatastore,
}

pub mod msg {
    use super::*;
    pub struct DepthSetSnapshot(pub Depth);
    pub struct DepthAddDiff(pub Depth);
    pub struct DepthGet;

    pub struct DepthMergeDiffsEvery(pub Duration);
    pub struct ExportToRedisEvery(pub Duration);
}

impl WsDatastoreActor {
    pub fn new(store: WsDatastore) -> Self {
        Self { store }
    }
}

impl Message<msg::DepthSetSnapshot> for WsDatastoreActor {
    type Reply = ();

    async fn handle(
        &mut self,
        msg::DepthSetSnapshot(depth): msg::DepthSetSnapshot,
        _ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        log::debug!("got msg DepthSetSnapshot: {}", self.store.name());
        self.store.depth_set_snapshot(depth);
    }
}

impl Message<msg::DepthAddDiff> for WsDatastoreActor {
    type Reply = ();

    async fn handle(
        &mut self,
        msg::DepthAddDiff(depth): msg::DepthAddDiff,
        _ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        log::debug!("got msg DepthAddDiff: {}", self.store.name());
        self.store.depth_add_diff(depth);
    }
}

impl Message<msg::DepthGet> for WsDatastoreActor {
    type Reply = Box<Depth>;

    async fn handle(
        &mut self,
        _msg: msg::DepthGet,
        _ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        log::debug!("got msg DepthGet: {}", self.store.name());
        self.store.depth_get().clone().into()
    }
}

#[cfg_attr(feature = "hotpath", hotpath::measure_all)]
impl Message<msg::DepthMergeDiffsEvery> for WsDatastoreActor {
    type Reply = ();

    async fn handle(
        &mut self,
        msg::DepthMergeDiffsEvery(dur): msg::DepthMergeDiffsEvery,
        ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        log::debug!("got msg DepthMergeDiffsEvery: {}", self.store.name());
        self.store.depth_merge_diffs();
        let actor_ref = ctx.actor_ref().clone();
        tokio::spawn(async move {
            tokio::time::sleep(dur).await;
            let _ = actor_ref.tell(msg::DepthMergeDiffsEvery(dur)).await;
        });
    }
}

impl Message<msg::ExportToRedisEvery> for WsDatastoreActor {
    type Reply = ();

    async fn handle(
        &mut self,
        msg::ExportToRedisEvery(dur): msg::ExportToRedisEvery,
        ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        log::debug!("got msg ExportToRedisEvery: {}", self.store.name());
        let _ = async {
            redis_store_actor::set(
                redis_store_actor::REDIS_STORE_DEFAULT_NAME,
                format!("{}:depth", self.store.name()),
                serde_json::json!(self.store.depth_get()).to_string(),
            )
            .await
        }
        .await
        .map_err(|e| log::warn!("ws_datastore: ExportToRedisEvery: {e:#?}"));

        let actor_ref = ctx.actor_ref().clone();
        tokio::spawn(async move {
            tokio::time::sleep(dur).await;
            let _ = actor_ref.tell(msg::ExportToRedisEvery(dur)).await;
        });
    }
}
