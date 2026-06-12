use anyhow::Result;

use crate::types::traits::KVStore;
use kameo::Actor;
use kameo::message::Message;

#[derive(Actor)]
pub struct KVStoreActor {
    store: Box<dyn KVStore>,
}

impl KVStoreActor {
    pub fn new(store: Box<dyn KVStore>) -> Self {
        Self { store }
    }
}

pub mod msg {
    use std::time::Duration;
    pub struct Set(pub String, pub String);
    pub struct Get(pub String);
    pub struct ReconnectEvery(pub Duration);
}

impl Message<msg::Get> for KVStoreActor {
    type Reply = Result<String>;

    async fn handle(
        &mut self,
        msg::Get(key): msg::Get,
        _ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        self.store.get(key).await
    }
}

impl Message<msg::Set> for KVStoreActor {
    type Reply = Result<()>;

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    async fn handle(
        &mut self,
        msg::Set(key, val): msg::Set,
        _ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        self.store.set(key, val).await
    }
}

impl Message<msg::ReconnectEvery> for KVStoreActor {
    type Reply = ();

    async fn handle(
        &mut self,
        msg::ReconnectEvery(dur): msg::ReconnectEvery,
        ctx: &mut kameo::prelude::Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let _ = self
            .store
            .reconnect()
            .map_err(|e| log::warn!("redis: ReconnectEvery: {e:#?}"));
        let actor_ref = ctx.actor_ref().clone();
        tokio::spawn(async move {
            tokio::time::sleep(dur).await;
            let _ = actor_ref.tell(msg::ReconnectEvery(dur)).await;
        });
    }
}
