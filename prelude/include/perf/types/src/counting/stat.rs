use alloc::collections::BTreeMap;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct CounterStat {
    pub event_id: u64,
    pub event_count: u64,
    pub time_enabled: u64,
    pub time_running: u64,
}

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct CounterGroupStat {
    pub time_enabled: u64,
    pub time_running: u64,
    /// Map of `event_id` -> `event_count`
    pub member_counts: BTreeMap<u64, u64>,
}
