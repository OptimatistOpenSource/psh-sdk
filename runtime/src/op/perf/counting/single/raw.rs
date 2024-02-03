use crate::op::perf::convert::Wrap;
use perf_event_rs::config;
use perf_event_rs::config::{Cpu as RawCpu, Process as RawProcess};
use perf_event_rs::counting::{Config as RawConfig, Counter, CounterStat};
use profiling_prelude_perf_types::config::{Cpu, Process};
use profiling_prelude_perf_types::counting::Config;
use std::io;

pub fn counter_new(process: &Process, cpu: &Cpu, cfg: &Config) -> config::Result<Counter> {
    let process = Wrap::<RawProcess>::from(process).into_inner();
    let cpu = Wrap::<RawCpu>::from(cpu).into_inner();
    let cfg = Wrap::<RawConfig>::from(cfg).into_inner();

    Counter::new(&process, &cpu, &cfg)
}

pub fn counter_enable(counter: &Counter) -> io::Result<()> {
    counter.enable()
}

pub fn counter_disable(counter: &Counter) -> io::Result<()> {
    counter.disable()
}

pub fn counter_reset(counter: &Counter) -> io::Result<()> {
    counter.reset()
}

pub fn counter_stat(counter: &mut Counter) -> io::Result<CounterStat> {
    counter.stat()
}