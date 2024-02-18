use crate::op;
use crate::op::test::{compile_profiling, gen_outs_errs_data};
use crate::profiling::runtime::ProfilingRuntime;
use crate::profiling::Profiling;
use std::fs;

fn gen_rt() -> ProfilingRuntime {
    let mut rt = ProfilingRuntime::new();

    #[rustfmt::skip]
    rt
    // intrinsics
    .link_op("log"          , op::intrinsics::log          ).unwrap()
    .link_op("log-err"      , op::intrinsics::log_err      ).unwrap()
    .link_op("exit"         , op::intrinsics::exit         ).unwrap()
    .link_op("drop-resource", op::intrinsics::drop_resource).unwrap();

    rt
}

#[test]
fn test_exit() {
    let bin_path = compile_profiling("../test-resources/profiling/exit");
    let wasm = fs::read(bin_path).unwrap();
    let profiling = unsafe { Profiling::from_precompiled(wasm) };
    let rt = gen_rt();

    let (outs, errs, data) = gen_outs_errs_data();
    let (_, r) = rt.run_profiling(data, &profiling);

    assert!(r.is_err());

    let outs = outs.lock().unwrap();
    assert_eq!(outs.len(), 1);
    assert_eq!(outs[0], "0");

    let errs = errs.lock().unwrap();
    assert_eq!(errs.len(), 0);
}

#[test]
fn test_log() {
    let bin_path = compile_profiling("../test-resources/profiling/log");
    let wasm = fs::read(bin_path).unwrap();
    let profiling = unsafe { Profiling::from_precompiled(wasm) };
    let rt = gen_rt();

    let (outs, errs, data) = gen_outs_errs_data();
    let (_, r) = rt.run_profiling(data, &profiling);

    assert!(r.is_ok());

    let outs = outs.lock().unwrap();
    assert_eq!(outs.len(), 3);
    assert_eq!(outs[0], "0");
    assert_eq!(outs[1], "1");
    assert_eq!(outs[2], "2");

    let errs = errs.lock().unwrap();
    assert_eq!(errs.len(), 0);
}

#[test]
fn test_panic() {
    let bin_path = compile_profiling("../test-resources/profiling/panic");
    let wasm = fs::read(bin_path).unwrap();
    let profiling = unsafe { Profiling::from_precompiled(wasm) };
    let rt = gen_rt();

    let (outs, errs, data) = gen_outs_errs_data();
    let (_, r) = rt.run_profiling(data, &profiling);

    assert!(r.is_err());

    let outs = outs.lock().unwrap();
    assert_eq!(outs.len(), 1);
    assert_eq!(outs[0], "0");

    let errs = errs.lock().unwrap();
    assert_eq!(errs.len(), 1);
    assert_eq!(
        errs[0],
        "Profiling panic: \npanicked at src/main.rs:11:5:\noops"
    );
}
