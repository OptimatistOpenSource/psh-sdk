use rkyv::{Archive, Deserialize, Serialize};
use crate::event::Event;

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct TracepointEvent {
    /// The content of `/sys/kernel/debug/tracing/events/*/*/id`
    pub id: u64,
}

impl TracepointEvent {
    pub const fn new(id: u64) -> Self {
        Self { id }
    }
}

impl From<TracepointEvent> for Event {
    fn from(value: TracepointEvent) -> Self {
        Self::Tracepoint(value)
    }
}
