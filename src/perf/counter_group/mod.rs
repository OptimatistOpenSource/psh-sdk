mod ty;
pub use ty::*;

pub trait CounterGroupExt {
    fn enable(self) -> Result<FixedCounterGroup, String>;
    fn into_fixed(self) -> Result<FixedCounterGroup, String>;
}

impl CounterGroupExt for CounterGroup {
    fn enable(self) -> Result<FixedCounterGroup, String> {
        CounterGroup::enable(self)
    }
    fn into_fixed(self) -> Result<FixedCounterGroup, String> {
        CounterGroup::into_fixed(self)
    }
}

impl CounterGroupStat {
    pub fn member_count(&self, guard: &CounterGuard) -> Option<u64> {
        self.member_counts
            .iter()
            .find(|(id, _)| *id == guard.event_id())
            .map(|(_, v)| *v)
    }
}
