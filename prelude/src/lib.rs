#![no_std]
#![cfg(target_arch = "wasm32")]

extern crate alloc;

mod pre_defined;

// TODO: auto pub use
pub use profiling_prelude_file as file;
pub use profiling_prelude_intrinsics as intrinsics;
pub use profiling_prelude_macros as macros;
pub use profiling_prelude_perf as perf;
