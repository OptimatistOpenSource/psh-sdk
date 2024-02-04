use crate::profiling::runtime::{Data, ProfilingRuntime};
use crate::profiling::Profiling;
use anyhow::{anyhow, Result};
use std::borrow::Borrow;
use wasmtime::{AsContextMut, Store, TypedFunc};
use wasmtime::{Instance, Module};

fn export_check(store: &mut Store<Data>, instance: Instance) -> Result<TypedFunc<(), ()>> {
    // main
    let main = instance.get_typed_func::<(), ()>(store.as_context_mut(), "main")?;

    // memory
    instance
        .get_export(store.as_context_mut(), "memory")
        .ok_or_else(|| anyhow!("No `memory` export"))
        .and_then(|it| {
            it.into_memory()
                .ok_or_else(|| anyhow!("Invalid `memory` export"))
        })?;

    // dealloc
    instance.get_typed_func::<(u32, u32), u32>(store.as_context_mut(), "alloc")?;

    // alloc
    instance.get_typed_func::<(u32, u32, u32), ()>(store.as_context_mut(), "dealloc")?;

    Ok(main)
}

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

        let instance = rt
            .wasm_linker
            .instantiate(&mut wasm_store, &wasm_module)
            .map_err(|e| (Data::new(), e))?;

        let main = export_check(&mut wasm_store, instance).map_err(|e| (Data::new(), e))?;

        let result = main.call(&mut wasm_store, ());

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
