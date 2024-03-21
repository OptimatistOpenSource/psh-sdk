use std::fs;
use std::process::Command;

fn main() {
    // Update git submodule
    Command::new("git")
        .args(["submodule", "update", "--init", "--recursive"])
        .status()
        .expect("Failed to update git submodule");

    // Generate bindings
    let _ = fs::remove_file("src/bindings.rs");
    Command::new("wit-bindgen")
        .args(["rust", "--out-dir", "src", "wit"])
        .status()
        .expect("Failed to generate bindings");
}
