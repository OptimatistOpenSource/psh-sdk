use crate::profiling::runtime::{Data, ProfilingRuntime};
use crate::profiling::Profiling;
use anyhow::Result;
use std::borrow::Borrow;
use wasmtime::Module;
use wasmtime::Store;

pub fn run_profiling(
    rt: &ProfilingRuntime,
    profiling: impl Borrow<Profiling>,
) -> (Data, Result<()>) {
    /*
    TODO: Refactor with try block
    Waiting for feature: https://github.com/rust-lang/rust/issues/31436
    */
    fn inner(rt: &ProfilingRuntime, profiling: &Profiling) -> Result<Data, (Data, anyhow::Error)> {
        let wasm_module = if profiling.is_aot {
            unsafe { Module::deserialize(&rt.wasm_engine, &profiling.bytes) }
        } else {
            Module::new(&rt.wasm_engine, &profiling.bytes)
        }
        .map_err(|e| (Data::new(), e))?;

        let mut wasm_store = {
            let data = Data::new(); // for log storage
            let mut store = Store::new(&rt.wasm_engine, data);
            store.set_epoch_deadline(1);
            store
        };

        let result = rt
            .wasm_linker
            .instantiate(&mut wasm_store, &wasm_module)
            .map_err(|e| (Data::new(), e))?
            .get_typed_func::<(), ()>(&mut wasm_store, "main")
            .map_err(|e| (Data::new(), e))?
            .call(&mut wasm_store, ());

        let data = wasm_store.into_data();
        match result {
            Ok(_) => Ok(data),
            Err(e) => Err((data, e)),
        }
    }

    let profiling = profiling.borrow();
    match inner(rt, profiling) {
        Ok(data) => (data, Ok(())),
        Err((data, e)) => (data, Err(e)),
    }
}
