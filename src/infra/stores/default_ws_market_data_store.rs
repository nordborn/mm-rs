use anyhow::Result;
use async_trait::async_trait;
use trading_types::Depth;

use crate::{
    tt_utils::liqs_util,
    types::traits::{KVStore, WsMarketDataStore},
};

pub struct DefaultWsMarketDataStore {
    name: String,
    depth_snapshot: Depth,
    depth_diffs: Vec<Depth>,
    /// external_kvstore is used for exporting purposes
    external_kvstore: Box<dyn KVStore>,
}

impl DefaultWsMarketDataStore {
    pub fn new(name: &str, external_kvstore: Box<dyn KVStore>) -> Self {
        Self {
            name: name.into(),
            depth_snapshot: Depth::new(),
            depth_diffs: Vec::new(),
            external_kvstore,
        }
    }

    fn depth_merge_diffs_sync(&mut self) {
        log::debug!("datastore: {}: merge diffs", self.name);
        let mut diff_asks = Vec::new();
        let mut diff_bids = Vec::new();
        for d in &self.depth_diffs {
            diff_asks.extend_from_slice(&d.asks);
            diff_bids.extend_from_slice(&d.bids);
        }
        let asks = liqs_util::merge_side_liqs(&self.depth_snapshot.asks, &diff_asks, true, 50);
        let bids = liqs_util::merge_side_liqs(&self.depth_snapshot.bids, &diff_bids, false, 50);
        self.depth_snapshot = Depth { asks, bids };
        self.depth_diffs = Vec::new();
    }

    fn depth_get_ref(&mut self) -> &Depth {
        self.depth_merge_diffs_sync();
        &self.depth_snapshot
    }
}

#[cfg_attr(feature = "hotpath", hotpath::measure_all)]
#[async_trait]
impl WsMarketDataStore for DefaultWsMarketDataStore {
    fn name(&self) -> &str {
        &self.name
    }

    fn depth_merge_diffs(&mut self) {
        log::debug!("market_datastore: {}: merge diffs", self.name);
        self.depth_merge_diffs_sync();
    }

    fn depth_set_snapshot(&mut self, d: Depth) {
        self.depth_snapshot = d;
    }

    fn depth_get(&mut self) -> Result<Box<Depth>> {
        Ok(Box::new(self.depth_get_ref().clone()))
    }

    fn depth_add_diff(&mut self, d: Depth) {
        self.depth_diffs.push(d);
    }

    async fn export_to_external_kvstore(&mut self) -> Result<()> {
        let name = self.name.clone();
        let value = serde_json::json!(self.depth_get_ref()).to_string();
        self.external_kvstore.set(format!("{name}:depth"), value).await
    }
}
