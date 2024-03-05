use wasmtime::component::{Linker, ResourceTable};

pub mod convert;
pub mod counting;

#[cfg(test)]
mod tests;

pub type Counter = perf_event_rs::counting::Counter;
pub type CounterGroup = perf_event_rs::counting::CounterGroup;
pub type FixedCounterGroup = perf_event_rs::counting::FixedCounterGroup;
pub type CounterGuard = perf_event_rs::counting::CounterGuard;

wasmtime::component::bindgen!({
    path: "wit",
    world: "imports",
    with: {
        "profiling:perf/counter/counter"                  : Counter,
        "profiling:perf/counter-group/counter-group"      : CounterGroup,
        "profiling:perf/counter-group/fixed-counter-group": FixedCounterGroup,
        "profiling:perf/counter-group/counter-guard"      : CounterGuard,
    }
});

pub struct PerfCtx {
    // TODO
}

#[allow(clippy::new_without_default)]
impl PerfCtx {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait PerfView {
    fn table(&self) -> &ResourceTable;
    fn table_mut(&mut self) -> &mut ResourceTable;
    fn ctx(&self) -> &PerfCtx;
    fn ctx_mut(&mut self) -> &mut PerfCtx;
}
