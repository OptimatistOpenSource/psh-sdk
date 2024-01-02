#![cfg_attr(feature = "profiling-dev", no_std)]

#[cfg(feature = "profiling-macros")]
pub use profiling_macros::*;
#[cfg(feature = "profiling-prelude")]
pub use profiling_prelude::*;
