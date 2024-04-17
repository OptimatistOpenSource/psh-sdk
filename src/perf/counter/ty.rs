#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::wit::profiling::perf;

// profiling:perf/counter
pub type Counter     = perf::counter::Counter;
pub type CounterStat = perf::counter::CounterStat;
