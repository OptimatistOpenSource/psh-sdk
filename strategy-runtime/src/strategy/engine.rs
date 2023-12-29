use std::borrow::Borrow;
use std::collections::VecDeque;

use anyhow::{Context, Result};
use wasmtime::{Config, Engine, IntoFunc, Linker, Module, Store, Trap};

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

    pub fn run_strategy(&self, strategy: impl Borrow<Strategy>) -> (Data, Result<()>) {
        /*
        TODO: Refactor with try block
        Waiting for feature: https://github.com/rust-lang/rust/issues/31436
        */
        fn inner(se: &StrategyEngine, strategy: &Strategy) -> Result<Data, (Data, anyhow::Error)> {
            let wasm_module = if strategy.is_aot {
                unsafe { Module::deserialize(&se.wasm_engine, &strategy.bytes) }
            } else {
                Module::new(&se.wasm_engine, &strategy.bytes)
            }.map_err(|e| (Data::new(), e))?;

            let mut wasm_store = {
                let data = Data::new(); // for log storage
                let mut store = Store::new(&se.wasm_engine, data);
                store.set_epoch_deadline(1);
                store
            };

            let result = se.wasm_linker
                .instantiate(&mut wasm_store, &wasm_module)
                .map_err(|e| (Data::new(), e))?
                .get_typed_func::<(), ()>(&mut wasm_store, "main")
                .map_err(|e| (Data::new(), e))?
                .call(&mut wasm_store, ());

            let data = wasm_store.into_data();
            match result {
                Ok(_) => Ok(data),
                Err(e) => Err((data, e))
            }
        }

        let strategy = strategy.borrow();
        match inner(self, strategy) {
            Ok(data) => (data, Ok(())),
            Err((data, e)) => (data, Err(e))
        }
    }
}
