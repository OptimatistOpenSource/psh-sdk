use crate::op::tests::{compile_profiling, gen_engine};
use crate::profiling::Profiling;
use std::fs;

#[test]
fn test_exit() {
    let bin_path = compile_profiling("../test-resources/profiling/exit");
    let wasm = fs::read(bin_path).unwrap();
    let profiling = unsafe { Profiling::from_precompiled(wasm) };
    let engine = gen_engine();

    let (data, r) = engine.run_profiling(profiling);
    assert!(r.is_err());
    let out = data.output_log();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], "0");
    let err = data.error_log();
    assert_eq!(err.len(), 0);
}

#[test]
fn test_log() {
    let bin_path = compile_profiling("../test-resources/profiling/log");
    let wasm = fs::read(bin_path).unwrap();
    let profiling = unsafe { Profiling::from_precompiled(wasm) };
    let engine = gen_engine();

    let (data, r) = engine.run_profiling(profiling);
    assert!(r.is_ok());
    let out = data.output_log();
    assert_eq!(out.len(), 3);
    assert_eq!(out[0], "0");
    assert_eq!(out[1], "1");
    assert_eq!(out[2], "2");
    let err = data.error_log();
    assert_eq!(err.len(), 0);
}

#[test]
fn test_panic() {
    let bin_path = compile_profiling("../test-resources/profiling/panic");
    let wasm = fs::read(bin_path).unwrap();
    let profiling = unsafe { Profiling::from_precompiled(wasm) };
    let engine = gen_engine();

    let (data, r) = engine.run_profiling(profiling);
    assert!(r.is_err());
    let out = data.output_log();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0], "0");
    let err = data.error_log();
    assert_eq!(err.len(), 1);
    assert_eq!(
        err[0],
        "Profiling panic: \npanicked at src/lib.rs:11:5:\noops"
    );
}
