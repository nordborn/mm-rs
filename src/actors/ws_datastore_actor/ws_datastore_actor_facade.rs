use std::time::Duration;

use kameo::actor::{ActorRef, Spawn};

use anyhow::{Context, Result};
use trading_types::{Depth, Symbol};

use super::WsDatastoreActor;
use crate::infra::stores::WsDatastore;

pub fn datastore_actor_name(symbol: &Symbol) -> String {
    format!("{symbol}:ws_datastore")
}

pub async fn spawn_register(register_name: &str, ws_datastore: WsDatastore) -> Result<ActorRef<WsDatastoreActor>> {
    let act = WsDatastoreActor::new(ws_datastore);
    let actor_ref = WsDatastoreActor::spawn(act);
    actor_ref.wait_for_startup().await;
    actor_ref
        .register(register_name.to_string())
        .with_context(|| format!("spawn_register: {register_name}"))?;
    Ok(actor_ref)
}

pub fn actor_ref(registered_name: &str) -> Result<ActorRef<WsDatastoreActor>> {
    ActorRef::lookup(registered_name)
        .context("registry")?
        .with_context(|| format!("ws_datastore: get_actor_ref: {registered_name}"))
}

pub async fn merge_diffs_every(registered_name: &str, dur: Duration) -> Result<()> {
    actor_ref(registered_name)?
        .tell(super::msg::DepthMergeDiffsEvery(dur))
        .await
        .context("merge_diffs_every")
}

pub async fn export_to_redis_every(registered_name: &str, dur: Duration) -> Result<()> {
    actor_ref(registered_name)?
        .tell(super::msg::ExportToRedisEvery(dur))
        .await
        .context("export_to_redis_every")
}

pub async fn depth_get(registered_name: &str) -> Result<Box<Depth>> {
    actor_ref(registered_name)?
        .ask(super::msg::DepthGet)
        .await
        .context("depth_get")
}

pub async fn depth_set_snapshot(registered_name: &str, depth: Depth) -> Result<()> {
    actor_ref(registered_name)?
        .ask(super::msg::DepthSetSnapshot(depth))
        .await
        .context("depth_set_snapshot")
}

pub async fn depth_add_diff(registered_name: &str, depth: Depth) -> Result<()> {
    actor_ref(registered_name)?
        .ask(super::msg::DepthAddDiff(depth))
        .await
        .context("depth_add_diff")
}

pub fn check_store_exists(registered_name: &str) -> bool {
    ActorRef::<WsDatastoreActor>::lookup(registered_name).unwrap().is_some()
}
