use profiling_runtime::profiling::runtime::{Data, ProfilingRuntime};
use std::fs;
use std::ops::Not;
use std::path::Path;
use std::process::Command;
use std::rc::Rc;
use std::sync::Mutex;

mod exit;
mod log;
mod panic;

fn compile_paot() {
    let mut cargo_build_paot = {
        let mut cmd = Command::new("cargo");
        cmd.args(["build", "--release", "--manifest-path", "../../aot/Cargo.toml"]);
        cmd
    };
    let output = cargo_build_paot.output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
}

pub fn compile_profiling(project_path: &str) -> String {
    let paot_path = "../../aot/target/release/paot";
    if Path::new(paot_path).exists().not() {
        compile_paot();
    }

    let _ = fs::remove_dir_all(format!("{}/target", project_path));
    let bin_name = project_path.replace('.', "_").replace('/', "-");
    let bin_path = format!("{}/target/{}", project_path, bin_name);
    let mut compile_profiling = {
        let mut cmd = Command::new(paot_path);
        cmd.args(["-p", project_path, "-o", bin_path.as_str()]);
        cmd
    };
    let output = compile_profiling.output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
    bin_path
}

pub fn gen_outs_errs_data() -> (Rc<Mutex<Vec<String>>>, Rc<Mutex<Vec<String>>>, Data) {
    let outs = Rc::new(Mutex::new(vec![]));
    let errs = Rc::new(Mutex::new(vec![]));
    let data = Data::new(
        {
            let outs = outs.clone();
            move |s: &str| {
                outs.lock().unwrap().push(s.to_string());
            }
        },
        {
            let errs = errs.clone();
            move |s: &str| {
                errs.lock().unwrap().push(s.to_string());
            }
        },
    );
    (outs, errs, data)
}

fn gen_rt() -> ProfilingRuntime {
    let mut rt = ProfilingRuntime::new();

    #[rustfmt::skip]
    rt
    // intrinsics
    .link_op("log"          , crate::log          ).unwrap()
    .link_op("log-err"      , crate::log_err      ).unwrap()
    .link_op("exit"         , crate::exit         ).unwrap()
    .link_op("drop-resource", crate::drop_resource).unwrap();

    rt
}
