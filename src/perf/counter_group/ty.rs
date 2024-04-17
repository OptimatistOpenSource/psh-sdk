#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::wit::profiling::perf;

// profiling:perf/counter-group
pub type CounterGroup      = perf::counter_group::CounterGroup;
pub type FixedCounterGroup = perf::counter_group::FixedCounterGroup;
pub type CounterGroupStat  = perf::counter_group::CounterGroupStat;
pub type CounterGuard      = perf::counter_group::CounterGuard;
