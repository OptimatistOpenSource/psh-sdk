use profiling::perf::*;

fn main() {
    let cfg = Config {
        event: HardwareEvent::CpuCycles.into(),
        scopes: EventScope::all(),
        extra_config: ExtraConfig::default(),
    };

    let counter = Counter::new(Process::Current, Cpu::Any, &cfg).unwrap();

    counter.enable().unwrap();
    println!("do something here...");
    counter.disable().unwrap();

    let stat = counter.stat().unwrap();

    println!("event_count: {}", stat.event_count);
    println!("time_enabled: {}", stat.time_enabled);
    println!("time_running: {}", stat.time_running);
    assert!(stat.event_count > 0);
    assert!(stat.time_enabled > 0);
    assert!(stat.time_running > 0);
}
