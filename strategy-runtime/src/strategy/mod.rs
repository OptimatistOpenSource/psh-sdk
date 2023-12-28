pub mod engine;

pub struct Strategy {
    pub(crate) is_aot: bool,
    pub(crate) bytes: Vec<u8>,
}

impl Strategy {
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
