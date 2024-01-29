use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct CounterStat {
    pub event_id: u64,
    pub event_count: u64,
    pub time_enabled: u64,
    pub time_running: u64,
}
