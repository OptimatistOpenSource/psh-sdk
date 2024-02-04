mod data;
mod run_profiling;

use std::borrow::Borrow;

pub use data::*;

use anyhow::Result;
use wasmtime::{Config, Engine, IntoFunc, Linker};

use crate::profiling::Profiling;

pub struct ProfilingRuntime {
    pub(crate) wasm_engine: Box<Engine>,
    pub(crate) wasm_linker: Linker<Data>,
}

impl Default for ProfilingRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfilingRuntime {
    pub fn new() -> Self {
        let wasm_engine = {
            let mut cfg = Config::new();
            cfg.epoch_interruption(true);
            let engine = Engine::new(&cfg).unwrap();
            Box::new(engine)
        };
        Self {
            wasm_linker: Linker::new(wasm_engine.as_ref()),
            wasm_engine,
        }
    }

    pub fn precompile_profiling(&self, profiling: Profiling) -> Result<Profiling> {
        if profiling.is_aot {
            return Ok(profiling);
        }

        let precompiled_bytes = self.wasm_engine.precompile_module(&profiling.bytes)?;

        let profiling = unsafe { Profiling::from_precompiled(precompiled_bytes) };
        Ok(profiling)
    }

    pub fn link_op<P, R>(&mut self, name: &str, f: impl IntoFunc<Data, P, R>) -> Result<&mut Self> {
        self.wasm_linker.func_wrap("op", name, f)?;
        Ok(self)
    }

    pub fn run_profiling(&self, profiling: impl Borrow<Profiling>) -> (Data, Result<()>) {
        run_profiling::run_profiling(self, profiling)
    }
}
