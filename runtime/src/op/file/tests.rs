use crate::op;
use crate::op::tests::compile_profiling;
use crate::profiling::runtime::ProfilingRuntime;
use crate::profiling::Profiling;
use std::fs;
use wasmtime::IntoFunc;

fn gen_engine() -> ProfilingRuntime {
    let mut engine = ProfilingRuntime::new();

    #[rustfmt::skip]
    engine
    // intrinsics
    .link_op("log"          , op::log              ).unwrap()
    .link_op("log-err"      , op::log_err          ).unwrap()
    .link_op("exit"         , op::exit             ).unwrap()
    .link_op("drop-resource", op::drop_resource    ).unwrap()
    // file
    .link_op("file-is-exist"   , op::file::is_exist   ).unwrap()
    .link_op("file-read"       , op::file::read       ).unwrap()
    .link_op("file-write"      , op::file::write      ).unwrap()
    .link_op("file-append"     , op::file::append     ).unwrap()
    .link_op("file-remove-file", op::file::remove_file).unwrap()
    .link_op("file-create-dir" , op::file::create_dir ).unwrap()
    .link_op("file-remove-dir" , op::file::remove_dir ).unwrap();

    engine
}

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
