use perf_event_rs::counting::{Config, Counter, CounterResult};
use perf_event_rs::{BuildError, Builder, EventScope, HwEvent};

#[repr(C)]
pub struct CounterConfig {
    pub calling_process: bool,
    pub pid: u32,
    pub any_cpu: bool,
    pub cpu: u32,

    pub event: u32,
    // TODO: add more options here...
}

pub fn new_counter(cfg: &CounterConfig) -> Result<Counter, BuildError> {
    let mut builder = Builder::new();

    if cfg.calling_process {
        builder = builder.calling_process();
    } else {
        builder = builder.on_process(cfg.pid)?;
    }

    if cfg.any_cpu {
        builder = builder.any_cpu();
    } else {
        builder = builder.on_cpu(cfg.cpu)?;
    }

    let event = match cfg.event {
        0 => HwEvent::CpuCycles,
        1 => HwEvent::Instructions,
        2 => HwEvent::CacheReferences,
        _ => todo!(),
    };

    let scopes = EventScope::all();
    let cfg = Config::new(event, scopes, Default::default());
    builder.build_counting(&cfg)
}

pub fn enable_counter(counter: &Counter) -> std::io::Result<()> {
    counter.enable()
}

pub fn disable_counter(counter: &Counter) -> std::io::Result<()> {
    counter.disable()
}

pub fn get_counter_result(counter: &mut Counter) -> std::io::Result<CounterResult> {
    counter.result()
}
