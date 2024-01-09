#![cfg(target_arch = "wasm32")]
#![no_std]

extern crate alloc;

use profiling::intrinsics;
use profiling::perf;
use profiling::macros;

#[profiling::main]
fn main() {
    let cfg = perf::CounterConfig {
        calling_process: true,
        pid: 0,
        any_cpu: true,
        cpu: 0,

        event: 1,
    };
    let counter = perf::Counter::new(&cfg).unwrap();

    counter.enable().unwrap();
    macros::println!("do something here...");
    counter.disable().unwrap();

    let result = counter.get_result().unwrap();

    macros::println!("event_count: {}",  result.event_count);
    macros::println!("time_enabled: {}", result.time_enabled);
    macros::println!("time_running: {}", result.time_running);
    assert!(result.event_count > 0);
    assert!(result.time_enabled > 0);
    assert!(result.time_running > 0);
}
