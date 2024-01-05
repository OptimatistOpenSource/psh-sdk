use crate::op::tests::{compile_profiling, gen_engine};
use crate::profiling::Profiling;
use std::fs;

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
