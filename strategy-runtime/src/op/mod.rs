use std::collections::VecDeque;

mod file;
mod perf;
pub mod wrapped;

#[inline]
pub fn log(info: String, ring_buffer: &mut VecDeque<String>) {
    ring_buffer.push_back(info);
}

#[inline]
pub fn exit() {}
