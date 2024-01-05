#![cfg(target_arch = "wasm32")]
#![no_std]

extern crate alloc;

use crate::alloc::string::ToString;
//use profiling::op;

use profiling_prelude::op;

//#[profiling::main]
#[profiling_macros::main]
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

    op::log(result.event_count.to_string());
    op::log(result.time_enabled.to_string());
    op::log(result.time_running.to_string());
    assert!(result.event_count > 0);
    assert!(result.time_enabled > 0);
    assert!(result.time_running > 0);
}
