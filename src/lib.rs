#[allow(clippy::all)]
mod bindings;

pub mod perf;
pub mod system;

#[inline]
pub fn name() -> String {
    bindings::name()
}
