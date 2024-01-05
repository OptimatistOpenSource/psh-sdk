use crate::op::tests::{compile_profiling, gen_engine};
use crate::profiling::Profiling;
use std::fs;

#[test]
fn test_file() {
    let bin_path = compile_profiling("../test-resources/profiling/file");
    let wasm = fs::read(bin_path).unwrap();
    let profiling = unsafe { Profiling::from_precompiled(wasm) };
    let engine = gen_engine();

    let tmp_dir = "../test-resources/tmp";
    let _ = fs::remove_dir(tmp_dir);
    let _ = fs::create_dir(tmp_dir);

    let (_, r) = engine.run_profiling(profiling);
    assert!(r.is_ok());
}
