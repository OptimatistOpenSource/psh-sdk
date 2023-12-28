use crate::args::Args;
use clap::Parser;
use colored::Colorize;
use std::fs;
use std::path::Path;
use std::process::Command;
use wasmtime::{Config, Engine};

mod args;

fn precompile_wasm(wasm_path: impl AsRef<Path>) -> Vec<u8> {
    let mut cfg = Config::new();
    cfg.epoch_interruption(true);
    let engine = Engine::new(&cfg).unwrap();
    let wasm = fs::read(wasm_path).unwrap();
    engine.precompile_module(&wasm).unwrap()
}

fn main() {
    let args: Args = Args::parse();
    match args {
        Args {
            project_dir: Some(project_dir),
            wasm_path: None,
            output_path: Some(output_path),
        } => {
            println!("  {}", "Compiling".green().bold());
            println!("    {} Cargo building...", "*".green());
            let cargo_toml_path = format!("{}/Cargo.toml", project_dir);
            let mut cargo_build = {
                let mut cmd = Command::new("cargo");
                cmd.args([
                    "build",
                    "--target",
                    "wasm32-unknown-unknown",
                    "--release",
                    "--manifest-path",
                    cargo_toml_path.as_str(),
                ]);
                cmd
            };
            cargo_build.output().unwrap();

            let wasm_path = format!(
                "{}/target/wasm32-unknown-unknown/release/strategy.wasm",
                project_dir
            );
            println!("    {} AOT compiling...", "*".green());
            let precompiled = precompile_wasm(wasm_path);
            println!("    {} Writing binary...", "*".green());
            fs::write(&output_path, precompiled).unwrap();
            println!("  {}", "Finished".green().bold());
            println!(
                "    {} Strategy was located in {}",
                "*".green(),
                output_path.green().bold()
            );
        }
        Args {
            project_dir: None,
            wasm_path: Some(wasm_path),
            output_path: Some(output_path),
        } => {
            println!("  {}", "Compiling".green().bold());
            println!("    {} AOT compiling...", "*".green());
            let precompiled = precompile_wasm(wasm_path);
            println!("    {} Writing binary...", "*".green());
            fs::write(&output_path, precompiled).unwrap();
            println!("  {}", "Finished".green().bold());
            println!(
                "    {} Strategy was located in {}",
                "*".green(),
                output_path.green().bold()
            );
        }
        _ => eprintln!("Invalid usage, try '--help' for more information"),
    }
}
