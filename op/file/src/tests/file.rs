use profiling_runtime::profiling::Profiling;
use std::fs;

use crate::tests::{compile_profiling, gen_outs_errs_data, gen_rt};

#[test]
fn test_file() {
    let bin_path = compile_profiling("../../test-resources/profiling/file");
    let wasm = fs::read(bin_path).unwrap();
    let profiling = unsafe { Profiling::from_precompiled(wasm) };
    let rt = gen_rt();

    let tmp_dir = "../../test-resources/tmp";
    let _ = fs::remove_dir(tmp_dir);
    let _ = fs::create_dir(tmp_dir);

    let (_, _, data) = gen_outs_errs_data();
    let (_, r) = rt.run_profiling(data, &profiling);
    assert!(r.is_ok());
}
