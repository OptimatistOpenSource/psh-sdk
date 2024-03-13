mod imports;

mod ty;

use std::ops::Not;
pub use ty::*;

impl Default for ExtraConfig {
    fn default() -> Self {
        Self {
            pinned: false,
            exclusive: false,
            inherit: false,
            inherit_stat: false,
            inherit_thread: false,
            enable_on_exec: false,
            remove_on_exec: false,
        }
    }
}

impl EventScope {
    pub fn all() -> Vec<EventScope> {
        vec![
            EventScope::User,
            EventScope::Kernel,
            EventScope::Hv,
            EventScope::Idle,
            EventScope::Host,
            EventScope::Guest,
        ]
    }

    pub fn all_but_exclude<'t>(scopes: impl IntoIterator<Item = &'t Self>) -> Vec<Self> {
        let excludes = scopes.into_iter().collect::<Vec<_>>();
        Self::all()
            .iter()
            .filter(|s| excludes.contains(s).not())
            .cloned()
            .collect()
    }
}

impl From<HardwareEvent> for Event {
    fn from(value: HardwareEvent) -> Self {
        Self::Hardware(value)
    }
}

impl From<SoftwareEvent> for Event {
    fn from(value: SoftwareEvent) -> Self {
        Self::Software(value)
    }
}

impl From<RawEvent> for Event {
    fn from(value: RawEvent) -> Self {
        Self::Raw(value)
    }
}

impl From<TracepointEvent> for Event {
    fn from(value: TracepointEvent) -> Self {
        Self::Tracepoint(value)
    }
}

impl From<BreakpointEvent> for Event {
    fn from(value: BreakpointEvent) -> Self {
        Self::Breakpoint(value)
    }
}

impl From<DynamicPmuEvent> for Event {
    fn from(value: DynamicPmuEvent) -> Self {
        Self::DynamicPmu(value)
    }
}

pub trait CounterGroupExt {
    fn enable(self) -> Result<FixedCounterGroup, String>;
}

impl CounterGroupExt for CounterGroup {
    fn enable(self) -> Result<FixedCounterGroup, String> {
        CounterGroup::enable(self)
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
