use crate::core::limit::Limit;
use crate::core::order::Order;

#[derive(Debug)]
pub struct Snapshot {
    pair: String,
}

pub struct SnapshotData {
    pair: String,
    pub orders: Vec<Order>,
    log_seq: i64,
    trade_seq: i64,
}

impl Snapshot {
    pub fn new(pair: String) -> Self {
        Self { pair }
    }

    pub fn construct_snapshot(
        &self,
        ask: Vec<Limit>,
        bid: Vec<Limit>,
        trade_seq: i64,
        log_seq: i64,
    ) -> SnapshotData {
        let mut i = 0;

        let mut snapshot_data = SnapshotData {
            orders: vec![],
            pair: self.pair.clone(),
            trade_seq,
            log_seq,
        };
        for limit in ask {
            for order in limit.orders {
                i += 1;
                snapshot_data.orders[i] = order;
            }
        }

        for limit in bid {
            for order in limit.orders {
                i += 1;
                snapshot_data.orders[i] = order;
            }
        }
        snapshot_data
    }
}
