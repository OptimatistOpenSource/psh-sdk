#![cfg_attr(feature = "strategy-dev", no_std)]

#[cfg(feature = "strategy-dev")]
pub use strategy_macros::*;
#[cfg(feature = "strategy-dev")]
pub use strategy_prelude::*;
