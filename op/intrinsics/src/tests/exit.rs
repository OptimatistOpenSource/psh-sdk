use std::fs;

use profiling_runtime::profiling::Profiling;

use crate::tests::{compile_profiling, gen_outs_errs_data, gen_rt};

#[test]
fn test_exit() {
    let bin_path = compile_profiling("../../test-resources/profiling/exit");
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

