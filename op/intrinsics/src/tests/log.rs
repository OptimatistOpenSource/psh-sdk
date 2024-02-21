use std::fs;

use profiling_runtime::profiling::Profiling;

use crate::tests::{compile_profiling, gen_outs_errs_data, gen_rt};

#[test]
fn test_log() {
    let bin_path = compile_profiling("../../test-resources/profiling/log");
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

