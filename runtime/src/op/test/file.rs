use crate::profiling::Profiling;
use std::fs;
use crate::op::test::{compile_profiling, gen_engine};

#[test]
fn test_file() {
    let bin_path = compile_profiling("../test-resources/profiling/file");
    let wasm = fs::read(bin_path).unwrap();
    let profiling = unsafe { Profiling::from_precompiled(wasm) };
    let engine = gen_engine();

    let tmp_dir = "../test-resources/tmp";
    let _ = fs::remove_dir(tmp_dir);
    let _ = fs::create_dir(tmp_dir);

    let (logs, r) = engine.run_profiling(profiling);
    //assert!(r.is_ok());
    //r.unwrap();
    //assert_eq!(logs.len(), 0);
    for log in logs {
        println!("{}", log);
    }
}
