use crate::op;
use crate::op::test::compile_profiling;
use crate::profiling::runtime::ProfilingRuntime;
use crate::profiling::Profiling;
use std::fs;

pub fn gen_engine() -> ProfilingRuntime {
    let mut engine = ProfilingRuntime::new();

    #[rustfmt::skip]
    engine
    // intrinsics
    .link_op("log"          , op::log          ).unwrap()
    .link_op("log-err"      , op::log_err      ).unwrap()
    .link_op("exit"         , op::exit         ).unwrap()
    .link_op("drop-resource", op::drop_resource).unwrap()
    // perf counter
    .link_op("perf-counter-new"    , op::perf::counter_new    ).unwrap()
    .link_op("perf-counter-enable" , op::perf::counter_enable ).unwrap()
    .link_op("perf-counter-disable", op::perf::counter_disable).unwrap()
    .link_op("perf-counter-reset"  , op::perf::counter_reset  ).unwrap()
    .link_op("perf-counter-stat"   , op::perf::counter_stat   ).unwrap()
    // perf counter group
    .link_op("perf-counter-group-new"       , op::perf::counter_group_new       ).unwrap()
    .link_op("perf-counter-group-add-member", op::perf::counter_group_add_member).unwrap()
    .link_op("perf-counter-group-enable"    , op::perf::counter_group_enable    ).unwrap()
    .link_op("perf-counter-group-stat"      , op::perf::counter_group_stat      ).unwrap()
    // perf fixed counter group
    .link_op("perf-fixed-counter-group-enable" ,op::perf::fixed_counter_group_enable ).unwrap()
    .link_op("perf-fixed-counter-group-disable",op::perf::fixed_counter_group_disable).unwrap()
    .link_op("perf-fixed-counter-group-reset"  ,op::perf::fixed_counter_group_reset  ).unwrap()
    .link_op("perf-fixed-counter-group-stat"   ,op::perf::fixed_counter_group_stat   ).unwrap()
    // perf counter guard
    .link_op("perf-counter-guard-event-id",op::perf::counter_guard_event_id).unwrap()
    .link_op("perf-counter-guard-stat"    ,op::perf::counter_guard_stat    ).unwrap();

    engine
}

#[test]
fn test_counter() {
    let bin_path = compile_profiling("../test-resources/profiling/perf-counter");
    let wasm = fs::read(bin_path).unwrap();
    let profiling = unsafe { Profiling::from_precompiled(wasm) };
    let engine = gen_engine();

    let (data, r) = engine.run_profiling(profiling);
    assert!(r.is_ok());
    let out = data.output_log();
    for log in out {
        print!("{}", log);
    }
    let err = data.error_log();
    assert_eq!(err.len(), 0);
}

#[test]
fn test_counter_group() {
    let bin_path = compile_profiling("../test-resources/profiling/perf-counter-group");
    let wasm = fs::read(bin_path).unwrap();
    let profiling = unsafe { Profiling::from_precompiled(wasm) };
    let engine = gen_engine();

    let (data, r) = engine.run_profiling(profiling);
    assert!(r.is_ok());
    let out = data.output_log();
    for log in out {
        print!("{}", log);
    }
    let err = data.error_log();
    assert_eq!(err.len(), 0);
}
