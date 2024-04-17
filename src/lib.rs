mod wit;

pub mod perf;
pub mod system;

#[inline]
pub fn name() -> String {
    wit::name()
}
