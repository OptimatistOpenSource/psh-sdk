use crate::profiling::runtime::{Data, ProfilingRuntime};
use crate::profiling::Profiling;
use anyhow::{anyhow, Result};
use wasmtime::{AsContextMut, Memory, Store, TypedFunc};
use wasmtime::{Instance, Module};

pub struct RequiredExports {
    pub main: TypedFunc<(), ()>,
    pub memory: Memory,
    pub alloc: TypedFunc<(u32, u32, u32), ()>,
    pub dealloc: TypedFunc<(u32, u32), u32>,
}

fn export_check(store: &mut Store<Data>, instance: Instance) -> Result<RequiredExports> {
    let main = instance.get_typed_func::<(), ()>(store.as_context_mut(), "main")?;

    let memory = instance
        .get_export(store.as_context_mut(), "memory")
        .ok_or_else(|| anyhow!("No `memory` export"))
        .and_then(|it| {
            it.into_memory()
                .ok_or_else(|| anyhow!("Invalid `memory` export"))
        })?;

    let alloc =
        instance.get_typed_func::<(u32, u32, u32), ()>(store.as_context_mut(), "dealloc")?;

    let dealloc = instance.get_typed_func::<(u32, u32), u32>(store.as_context_mut(), "alloc")?;

    Ok(RequiredExports {
        main,
        memory,
        alloc,
        dealloc,
    })
}

pub fn run_profiling(
    rt: &ProfilingRuntime,
    data: Data,
    profiling: &Profiling,
) -> (Data, Result<()>) {
    let module = if profiling.is_aot {
        unsafe { Module::deserialize(&rt.engine, &profiling.bytes) }
    } else {
        Module::new(&rt.engine, &profiling.bytes)
    };
    let module = match module {
        Ok(it) => it,
        Err(e) => return (data, Err(e)),
    };

    let mut store = {
        let mut store = Store::new(&rt.engine, data);
        store.set_epoch_deadline(1);
        store
    };

    let instance = rt.linker.instantiate(&mut store, &module);
    let instance = match instance {
        Ok(it) => it,
        Err(e) => return (store.into_data(), Err(e)),
    };

    let required_exports = export_check(&mut store, instance);
    let required_exports = match required_exports {
        Ok(it) => it,
        Err(e) => return (store.into_data(), Err(e)),
    };

    match required_exports.main.call(&mut store, ()) {
        Ok(_) => (store.into_data(), Ok(())),
        Err(e) => (store.into_data(), Err(e)),
    }
}
