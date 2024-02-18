#![cfg(target_arch = "wasm32")]
#![no_std]

extern crate alloc;

use core::assert;
use core::assert_eq;

use prelude::intrinsics;
use prelude::macros::*;
use prelude::perf::config::*;
use prelude::perf::counting::*;
use prelude::perf::event::*;
use prelude::proc_macros::main;
use profiling::prelude;

#[main]
fn main() {
    let cfg = Config {
        event: HardwareEvent::CpuCycles.into(),
        scopes: EventScope::all(),
        extra_config: Default::default(),
    };

    // Test counter_group_new
    let counter_group = CounterGroup::new(&Process::Current, &Cpu::Any).unwrap();

    // Test counter_group_add_member
    let cpu_cycles_guard = counter_group.add_member(&cfg).unwrap();
    let instructions_guard = counter_group.add_member(&cfg).unwrap();

    // Test counter_group_stat
    let stat = counter_group.stat().unwrap();
    assert_eq!(stat.time_enabled, 0);
    assert_eq!(stat.time_running, 0);
    let cpu_cycles = stat.member_count(&cpu_cycles_guard).unwrap();
    let instructions = stat.member_count(&instructions_guard).unwrap();
    assert_eq!(cpu_cycles, 0);
    assert_eq!(instructions, 0);

    // Test counter_group_enable
    let fixed_counter_group = counter_group.enable().unwrap();
    println!("do something here...");
    fixed_counter_group.disable().unwrap();

    // Test fixed_counter_group_stat
    let stat = fixed_counter_group.stat().unwrap();
    assert!(stat.time_enabled > 0);
    assert!(stat.time_running > 0);
    let cpu_cycles = stat.member_count(&cpu_cycles_guard).unwrap();
    let instructions = stat.member_count(&instructions_guard).unwrap();
    assert!(cpu_cycles > 0);
    assert!(instructions > 0);
    println!("time_enabled: {}", stat.time_enabled);
    println!("time_running: {}", stat.time_running);
    println!("cpu cycles / instructions = {} / {}", cpu_cycles,instructions);

    // Test fixed_counter_group_disable
    assert_eq!(fixed_counter_group.stat().unwrap().time_enabled, stat.time_enabled);

    // Test fixed_counter_group_reset
    fixed_counter_group.reset().unwrap();
    let stat = fixed_counter_group.stat().unwrap();
    let cpu_cycles = stat.member_count(&cpu_cycles_guard).unwrap();
    let instructions = stat.member_count(&instructions_guard).unwrap();
    assert_eq!(cpu_cycles, 0);
    assert_eq!(instructions, 0);

    // Test fixed_counter_group_enable
    fixed_counter_group.enable().unwrap();
    let stat = fixed_counter_group.stat().unwrap();
    assert!(stat.time_enabled > 0);
    assert!(stat.time_running > 0);
    let cpu_cycles = stat.member_count(&cpu_cycles_guard).unwrap();
    let instructions = stat.member_count(&instructions_guard).unwrap();
    assert!(cpu_cycles > 0);
    assert!(instructions > 0);
}
