use std::time::Duration;

use kameo::actor::{ActorRef, Spawn};

use anyhow::{Context, Result};

use super::RedisStoreActor;
use crate::infra::stores::RedisStore;

pub const REDIS_STORE_DEFAULT_NAME: &str = "redis_store_default";

pub async fn spawn_register(register_name: &str, redis_store: RedisStore) -> Result<ActorRef<RedisStoreActor>> {
    let act = RedisStoreActor::new(redis_store);
    let actor_ref = RedisStoreActor::spawn(act);
    actor_ref.wait_for_startup().await;
    actor_ref
        .register(register_name.to_string())
        .with_context(|| format!("redis spawn_register: {register_name}"))?;
    Ok(actor_ref)
}

#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn actor_ref(registered_name: &str) -> Result<ActorRef<RedisStoreActor>> {
    ActorRef::lookup(registered_name)
        .context("registry")?
        .with_context(|| format!("redis_store: get_actor_ref: {registered_name}"))
}

pub async fn get(registered_name: &str, key: String) -> Result<String> {
    actor_ref(registered_name)?
        .ask(super::msg::Get(key))
        .await
        .context("get")
}

#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub async fn set(registered_name: &str, key: String, value: String) -> Result<()> {
    actor_ref(registered_name)?
        .tell(super::msg::Set(key, value)) // <--
        .await
        .context("set")
}

pub async fn reconnect_every(registered_name: &str, dur: Duration) -> Result<()> {
    actor_ref(registered_name)?
        .tell(super::msg::ReconnectEvery(dur))
        .await
        .context("reconnect_every")
}
