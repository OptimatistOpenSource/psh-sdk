use crate::op;
use crate::op::test::{compile_profiling, gen_outs_errs_data};
use crate::profiling::runtime::ProfilingRuntime;
use crate::profiling::Profiling;
use std::fs;

pub fn gen_rt() -> ProfilingRuntime {
    let mut rt = ProfilingRuntime::new();

    #[rustfmt::skip]
    rt
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

    rt
}

#[test]
fn test_counter() {
    let bin_path = compile_profiling("../test-resources/profiling/perf-counter");
    let wasm = fs::read(bin_path).unwrap();
    let profiling = unsafe { Profiling::from_precompiled(wasm) };
    let rt = gen_rt();

    let (outs, errs, data) = gen_outs_errs_data();
    let (_, r) = rt.run_profiling(data, &profiling);

    assert!(r.is_ok());

    let outs = outs.lock().unwrap();
    for out in outs.iter() {
        print!("{}", out);
    }

    let errs = errs.lock().unwrap();
    assert_eq!(errs.len(), 0);
}

#[test]
fn test_counter_group() {
    let bin_path = compile_profiling("../test-resources/profiling/perf-counter-group");
    let wasm = fs::read(bin_path).unwrap();
    let profiling = unsafe { Profiling::from_precompiled(wasm) };
    let rt = gen_rt();

    let (outs, errs, data) = gen_outs_errs_data();
    let (_, r) = rt.run_profiling(data, &profiling);

    assert!(r.is_ok());

    let outs = outs.lock().unwrap();
    for out in outs.iter() {
        print!("{}", out);
    }

    let errs = errs.lock().unwrap();
    assert_eq!(errs.len(), 0);
}
