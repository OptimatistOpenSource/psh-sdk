#![cfg(target_arch = "wasm32")]
#![no_std]

extern crate alloc;

use profiling::op;
use profiling::println;

#[profiling::main]
fn main() {
    let cfg = op::perf::CounterConfig {
        calling_process: true,
        pid: 0,
        any_cpu: true,
        cpu: 0,

        event: 1,
    };
    let counter = op::perf::Counter::new(&cfg).unwrap();

    counter.enable().unwrap();
    op::log("do something here...");
    counter.disable().unwrap();

    let result = counter.get_result().unwrap();

    println!("event_count: {}",  result.event_count);
    println!("time_enabled: {}", result.time_enabled);
    println!("time_running: {}", result.time_running);
    assert!(result.event_count > 0);
    assert!(result.time_enabled > 0);
    assert!(result.time_running > 0);
}
