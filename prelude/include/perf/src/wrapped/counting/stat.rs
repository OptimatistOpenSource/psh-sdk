use crate::counting::CounterGuard;
use alloc::string::{String, ToString};
use profiling_prelude_perf_types::counting::CounterGroupStat;

pub trait CounterGroupStatExt {
    fn member_count(&self, counter_guard: &CounterGuard) -> Result<u64, String>;
}

impl CounterGroupStatExt for CounterGroupStat {
    fn member_count(&self, counter_guard: &CounterGuard) -> Result<u64, String> {
        let event_id = counter_guard.event_id()?;
        match self.member_counts.get(&event_id) {
            Some(event_count) => Ok(*event_count),
            None => Err("No event count found for this CounterGuard".to_string()),
        }
    }
}
