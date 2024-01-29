#![cfg_attr(feature = "prelude", no_std)]

#[cfg(feature = "prelude")]
pub use profiling_prelude as prelude;

#[cfg(feature = "runtime")]
pub use profiling_runtime as runtime;
