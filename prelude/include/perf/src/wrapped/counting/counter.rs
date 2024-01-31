use crate::bindings::op::*;
use alloc::string::String;
use core::mem::MaybeUninit;
use profiling_prelude_perf_types::config::{Cpu, Process};
use profiling_prelude_perf_types::counting::{Config, CounterStat};
use profiling_prelude_perf_types::{claim_raw_parts_de, ser};

pub struct Counter {
    rid: u32,
}

impl Counter {
    pub fn new(process: &Process, cpu: &Cpu, cfg: &Config) -> Result<Self, String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;

        let sered_process = ser(process);
        let sered_process_ptr = sered_process.as_ptr();
        let sered_process_len = sered_process.len();

        let sered_cpu = ser(cpu);
        let sered_cpu_ptr = sered_cpu.as_ptr();
        let sered_cpu_len = sered_cpu.len();

        let sered_cfg = ser(cfg);
        let sered_cfg_ptr = sered_cfg.as_ptr();
        let sered_cfg_len = sered_cfg.len();

        perf_counter_new(
            ret_area_ptr,
            sered_process_ptr as _,
            sered_process_len as _,
            sered_cpu_ptr as _,
            sered_cpu_len as _,
            sered_cfg_ptr as _,
            sered_cfg_len as _,
        );

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
        perf_counter_enable(ret_area_ptr, self.rid);

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
        perf_counter_disable(ret_area_ptr, self.rid);

        let [is_ok, ptr, len] = ret_area;
        match is_ok {
            1 => Ok(()),
            0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }

    pub fn reset_count(&self) -> Result<(), String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;
        perf_counter_reset_count(ret_area_ptr, self.rid);

        let [is_ok, ptr, len] = ret_area;
        match is_ok {
            1 => Ok(()),
            0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }

    pub fn stat(&self) -> Result<CounterStat, String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;
        perf_counter_stat(ret_area_ptr, self.rid);

        let [is_ok, ptr, len] = ret_area;
        match is_ok {
            1 => {
                let cr: CounterStat = unsafe { claim_raw_parts_de(ptr as _, len as _) };
                Ok(cr)
            }
            0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        profiling_prelude_intrinsics::drop_resource(self.rid)
    }
}
