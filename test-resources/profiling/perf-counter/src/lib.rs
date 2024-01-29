#![cfg(target_arch = "wasm32")]
#![no_std]

extern crate alloc;

use profiling::prelude;
use prelude::intrinsics;
use prelude::macros::*;
use prelude::proc_macros::main;
use prelude::perf::counting::*;
use prelude::perf::event::*;

#[main]
fn main() {
    let cfg = Config {
        cpu: Cpu::Any,
        process: Process::Calling,
        event: HardwareEvent::CpuCycles.into(),
        scopes: EventScope::all(),
        extra_config: Default::default(),
    };

    let counter = Counter::new(&cfg).unwrap();

    counter.enable().unwrap();
    println!("do something here...");
    counter.disable().unwrap();

    let result = counter.stat().unwrap();

    println!("event_count: {}", result.event_count);
    println!("time_enabled: {}", result.time_enabled);
    println!("time_running: {}", result.time_running);
    assert!(result.event_count > 0);
    assert!(result.time_enabled > 0);
    assert!(result.time_running > 0);
}
