pub mod engine;

pub struct Profiling {
    pub(crate) is_aot: bool,
    pub(crate) bytes: Vec<u8>,
}

impl Profiling {
    pub fn from_wasm(bytes: Vec<u8>) -> Self {
        Self {
            is_aot: false,
            bytes,
        }
    }

    pub unsafe fn from_precompiled(bytes: Vec<u8>) -> Self {
        Self {
            is_aot: true,
            bytes,
        }
    }
}
