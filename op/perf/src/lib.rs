use wasmtime::component::{Linker, ResourceTable};

pub mod convert;
pub mod counting;

#[cfg(test)]
mod tests;

pub struct PerfCtx {
    // TODO
}

#[allow(clippy::new_without_default)]
impl PerfCtx {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait PerfView {
    fn table(&self) -> &ResourceTable;
    fn table_mut(&mut self) -> &mut ResourceTable;
    fn ctx(&self) -> &PerfCtx;
    fn ctx_mut(&mut self) -> &mut PerfCtx;
}
