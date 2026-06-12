use anyhow::{Context, Result};
use async_trait::async_trait;
use std::time::Duration;

use crate::{
    infra::actors::kvstore_actor::{KVStoreActor, msg},
    types::traits::KVStore,
};
use kameo::actor::{ActorRef, Spawn};

// pub const REDIS_STORE_DEFAULT_NAME: &str = "redis_store_default";

#[derive(Debug, Clone)]
pub struct KVStoreActorAdapter {
    pub name: String,
    pub url: String,
    pub reconnect_every: Duration,
}

impl KVStoreActorAdapter {
    pub async fn try_new(name: &str, url: &str, reconnect_every: Duration, kvstore: Box<dyn KVStore>) -> Result<Self> {
        let ctx = format!("MemstoreActorAdapter: try_new: {name}");
        let obj = Self {
            name: name.into(),
            url: url.into(),
            reconnect_every,
        };
        obj.try_init(kvstore).await.with_context(|| ctx.clone())?;
        Ok(obj)
    }

    pub async fn try_init(&self, kvstore: Box<dyn KVStore>) -> Result<()> {
        let name = &self.name;
        let ctx = format!("RedisMemStoreAdapter: try_init: {name}");
        // let redis_store = RedisStore::try_new(&self.url).with_context(|| ctx.clone())?;
        let act = KVStoreActor::new(kvstore);
        let actor_ref = KVStoreActor::spawn(act);
        actor_ref.wait_for_startup().await;
        actor_ref.register(name.clone()).with_context(|| ctx.clone())?;
        self.on_init_reconnect_every().await.with_context(|| ctx)?;
        Ok(())
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn actor_ref(&self) -> Result<ActorRef<KVStoreActor>> {
        let name = self.name.as_str();
        ActorRef::lookup(name)
            .context("registry")?
            .with_context(|| format!("get_actor_ref: {name}"))
    }

    async fn on_init_reconnect_every(&self) -> Result<()> {
        self.actor_ref()?
            .tell(msg::ReconnectEvery(self.reconnect_every))
            .await
            .context("reconnect_every")
    }
}

#[async_trait]
impl KVStore for KVStoreActorAdapter {
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    async fn get(&mut self, key: String) -> Result<String> {
        self.actor_ref()?.ask(msg::Get(key)).await.context("get")
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    async fn set(&mut self, key: String, value: String) -> Result<()> {
        self.actor_ref()?
            .tell(msg::Set(key, value)) // <--
            .await
            .context("set")
    }

    /// internally reconnects respecting `reconnect_every`
    fn reconnect(&mut self) -> Result<()> {
        Ok(())
    }
}
