#![allow(unused_imports)]

mod cpu;
mod event;
mod event_scope;
mod extra_config;
mod process;
mod stat;
mod config;

pub use cpu::*;
pub use event::*;
pub use event_scope::*;
pub use extra_config::*;
pub use process::*;
pub use stat::*;
pub use config::*;

#[repr(transparent)]
pub struct Wrap<T>(T);

impl<T> Wrap<T> {
    pub fn unwrap(self) -> T {
        self.0
    }
}
