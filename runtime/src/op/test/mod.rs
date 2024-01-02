use crate::op;
use crate::profiling::runtime::ProfilingEngine;
use std::process::Command;

mod file;
mod intrinsics;
mod perf;

pub fn gen_engine() -> ProfilingEngine {
    let mut engine = ProfilingEngine::new();
    engine
        .link_op("log", op::log)
        .unwrap()
        .link_op("exit", op::exit)
        .unwrap()
        .link_op("file-is-exist", op::file::is_exist)
        .unwrap()
        .link_op("file-read", op::file::read)
        .unwrap()
        .link_op("file-write", op::file::write)
        .unwrap()
        .link_op("file-append", op::file::append)
        .unwrap()
        .link_op("file-remove-file", op::file::remove_file)
        .unwrap()
        .link_op("file-create-dir", op::file::create_dir)
        .unwrap()
        .link_op("file-remove-dir", op::file::remove_dir)
        .unwrap();
    engine
}

fn compile_paot() {
    let mut cargo_build_paot = {
        let mut cmd = Command::new("cargo");
        cmd.args(["build", "--release", "--manifest-path", "../aot/Cargo.toml"]);
        cmd
    };
    let output = cargo_build_paot.output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
}

pub fn compile_profiling(project_path: &str) -> String {
    compile_paot();

    let bin_name = project_path.replace('.', "_").replace('/', "-");
    let bin_path = format!("{}/target/{}", project_path, bin_name);
    let mut compile_profiling = {
        let paot_path = "../aot/target/release/paot";
        let mut cmd = Command::new(paot_path);
        cmd.args(["-p", project_path, "-o", bin_path.as_str()]);
        cmd
    };
    let output = compile_profiling.output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
    bin_path
}
