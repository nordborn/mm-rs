use trading_types::Depth;

use crate::tt_utils::liqs_util;

#[derive(Debug)]
pub struct WsDatastore {
    name: String,
    depth_snapshot: Depth,
    depth_diffs: Vec<Depth>,
}

impl WsDatastore {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            depth_snapshot: Depth::new(),
            depth_diffs: Vec::new(),
        }
    }
}

#[cfg_attr(feature = "hotpath", hotpath::measure_all)]
impl WsDatastore {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn depth_set_snapshot(&mut self, d: Depth) {
        self.depth_snapshot = d;
    }

    pub fn depth_get(&mut self) -> &Depth {
        self.depth_merge_diffs();
        &self.depth_snapshot
    }

    pub fn depth_add_diff(&mut self, d: Depth) {
        self.depth_diffs.push(d);
    }

    pub fn depth_merge_diffs(&mut self) {
        log::debug!("ws_datastore: {}: merge diffs", self.name);
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
}
