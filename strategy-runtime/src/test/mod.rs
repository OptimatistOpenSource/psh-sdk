mod op;

use crate::strategy::engine::StrategyEngine;
use crate::strategy::Strategy;
use std::process::Command;
use std::{fs, panic};

pub fn gen_engine() -> StrategyEngine {
    use crate::op::wrapped as op;
    let mut engine = StrategyEngine::new();
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

fn compile_sc() {
    let mut cargo_build_sc = {
        let mut cmd = Command::new("cargo");
        cmd.args([
            "build",
            "--release",
            "--manifest-path",
            "../strategy-compiler/Cargo.toml",
        ]);
        cmd
    };
    let output = cargo_build_sc.output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
}

pub fn compile_strategy(project_path: &str) -> String {
    compile_sc();
    let bin_name = project_path.replace('.', "_").replace('/', "-");
    let bin_path = format!("{}/target/{}", project_path, bin_name);
    let mut compile_strategy = {
        let sc_path = "../strategy-compiler/target/release/sc";
        let mut cmd = Command::new(sc_path);
        cmd.args(["-p", project_path, "-o", bin_path.as_str()]);
        cmd
    };
    let output = compile_strategy.output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
    bin_path
}
