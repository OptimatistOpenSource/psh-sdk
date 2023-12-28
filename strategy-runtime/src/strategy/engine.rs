use std::borrow::Borrow;
use std::collections::VecDeque;

use anyhow::Result;
use wasmtime::{Config, Engine, IntoFunc, Linker, Module, Store};

use crate::strategy::Strategy;

pub type Data = VecDeque<String>;

pub struct StrategyEngine {
    pub(crate) wasm_engine: Box<Engine>,
    pub(crate) wasm_linker: Linker<Data>,
}

impl StrategyEngine {
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

    pub fn precompile_strategy(&self, strategy: Strategy) -> Result<Strategy> {
        if strategy.is_aot {
            return Ok(strategy);
        }

        let precompiled_bytes = self.wasm_engine.precompile_module(&strategy.bytes)?;

        let strategy = unsafe { Strategy::from_precompiled(precompiled_bytes) };
        Ok(strategy)
    }

    pub fn link_op<P, R>(&mut self, name: &str, f: impl IntoFunc<Data, P, R>) -> Result<&mut Self> {
        self.wasm_linker.func_wrap("op", name, f)?;
        Ok(self)
    }

    pub fn run_strategy(&self, strategy: impl Borrow<Strategy>) -> Result<Data> {
        let strategy = strategy.borrow();
        let wasm_module = if strategy.is_aot {
            unsafe { Module::deserialize(&self.wasm_engine, &strategy.bytes) }
        } else {
            Module::new(&self.wasm_engine, &strategy.bytes)
        }?;

        let data = Data::new(); // for log storage
        let mut wasm_store = Store::new(&self.wasm_engine, data);
        wasm_store.set_epoch_deadline(1);

        self.wasm_linker
            .instantiate(&mut wasm_store, &wasm_module)?
            .get_typed_func::<(), ()>(&mut wasm_store, "main")?
            .call(&mut wasm_store, ())?;

        Ok(wasm_store.into_data())
    }
}
