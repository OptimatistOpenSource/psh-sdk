#![cfg(target_arch = "wasm32")]
#![no_std]

extern crate alloc;

use profiling::prelude;
use prelude::intrinsics;
use prelude::macros::*;
use prelude::perf;

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
    println!("do something here...");
    counter.disable().unwrap();

    let result = counter.get_result().unwrap();

    println!("event_count: {}",  result.event_count);
    println!("time_enabled: {}", result.time_enabled);
    println!("time_running: {}", result.time_running);
    assert!(result.event_count > 0);
    assert!(result.time_enabled > 0);
    assert!(result.time_running > 0);
}