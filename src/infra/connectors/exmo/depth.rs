use anyhow::{Context, Result};
use trading_types::{Depth, Symbol};

use crate::{actors::ws_datastore_actor, types::traits::DepthFetcher};

#[async_trait::async_trait]
impl DepthFetcher for super::Exmo {
    // gets it from datastore, no real fetch needed
    async fn fetch_depth(&self, symbol: &Symbol) -> Result<Depth> {
        let ctx = format!("fetch_depth: {symbol}");
        let store_actor_name = ws_datastore_actor::datastore_actor_name(symbol);
        let depth = ws_datastore_actor::depth_get(&store_actor_name)
            .await
            .with_context(|| ctx)?;
        Ok(*depth)
    }
}
