use crate::op::bindings::op::*;
use alloc::boxed::Box;
use alloc::string::String;
use core::mem::MaybeUninit;

#[repr(C)]
pub struct CounterConfig {
    pub calling_process: bool,
    pub pid: u32,
    pub any_cpu: bool,
    pub cpu: u32,

    pub event: u32,
    // TODO: add more options here...
}

#[repr(C)]
pub struct CounterResult {
    pub event_count: u64,
    pub time_enabled: u64,
    pub time_running: u64,
}

pub struct Counter {
    rid: u32,
}

impl Counter {
    pub fn new(cfg: &CounterConfig) -> Result<Self, String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;

        // TODO: serialize here...
        let serialized_cfg_ptr = cfg as *const _;
        let serialized_cfg_len = 0;
        perf_new_counter(ret_area_ptr, serialized_cfg_ptr as _, serialized_cfg_len);

        let [is_ok, rid_or_ptr, len] = ret_area;
        match is_ok {
            1 => Ok(Self { rid: rid_or_ptr }),
            0 => Err(unsafe { String::from_raw_parts(rid_or_ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }

    pub fn enable(&self) -> Result<(), String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;
        perf_enable_counter(ret_area_ptr, self.rid);

        let [is_ok, ptr, len] = ret_area;
        match is_ok {
            1 => Ok(()),
            0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }

    pub fn disable(&self) -> Result<(), String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;
        perf_disable_counter(ret_area_ptr, self.rid);

        let [is_ok, ptr, len] = ret_area;
        match is_ok {
            1 => Ok(()),
            0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }

    pub fn get_result(&self) -> Result<CounterResult, String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;
        perf_get_counter_result(ret_area_ptr, self.rid);

        let [is_ok, ptr, len] = ret_area;
        match is_ok {
            1 => {
                // TODO: deserialize here...
                let _ = len;
                let cr = unsafe { Box::<CounterResult>::from_raw(ptr as _) };
                Ok(*cr)
            }
            0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        crate::op::drop_resource(self.rid)
    }
}
