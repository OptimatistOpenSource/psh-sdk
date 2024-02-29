use crate::bindings::op::*;
use crate::counting::CounterGuard;
use crate::counting::FixedCounterGroup;
use alloc::string::String;
use core::mem::MaybeUninit;
use core::ops::Not;
use profiling_prelude_perf_types::claim_raw_parts_de;
use profiling_prelude_perf_types::config::{Cpu, Process};
use profiling_prelude_perf_types::counting::Config;
use profiling_prelude_perf_types::counting::CounterGroupStat;
use profiling_prelude_perf_types::ser;

pub struct CounterGroup {
    /// Since [`enable`] will drop `rid` in the host side,
    /// only drop if `is_dropped` is `false` to avoid double free.
    is_dropped: bool,
    rid: u32,
}

impl CounterGroup {
    pub fn new(process: &Process, cpu: &Cpu) -> Result<Self, String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;

        let sered_process = ser(process);
        let sered_process_ptr = sered_process.as_ptr();
        let sered_process_len = sered_process.len();

        let sered_cpu = ser(cpu);
        let sered_cpu_ptr = sered_cpu.as_ptr();
        let sered_cpu_len = sered_cpu.len();

        perf_counter_group_new(
            ret_area_ptr,
            sered_process_ptr as _,
            sered_process_len as _,
            sered_cpu_ptr as _,
            sered_cpu_len as _,
        );

        let [is_ok, rid_or_ptr, len] = ret_area;
        match is_ok {
            1 => Ok(Self {
                is_dropped: false,
                rid: rid_or_ptr,
            }),
            0 => Err(unsafe { String::from_raw_parts(rid_or_ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }

    pub fn add_member(&self, cfg: &Config) -> Result<CounterGuard, String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;

        let sered_cfg = ser(cfg);
        let sered_cfg_ptr = sered_cfg.as_ptr();
        let sered_cfg_len = sered_cfg.len();

        perf_counter_group_add_member(
            ret_area_ptr,
            self.rid,
            sered_cfg_ptr as _,
            sered_cfg_len as _,
        );

        let [is_ok, rid_or_ptr, len] = ret_area;
        match is_ok {
            1 => Ok(CounterGuard { rid: rid_or_ptr }),
            0 => Err(unsafe { String::from_raw_parts(rid_or_ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }

    pub fn enable(mut self) -> Result<FixedCounterGroup, String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;

        perf_counter_group_enable(ret_area_ptr, self.rid);
        self.is_dropped = true;

        let [is_ok, rid_or_ptr, len] = ret_area;
        match is_ok {
            1 => Ok(FixedCounterGroup { rid: rid_or_ptr }),
            0 => Err(unsafe { String::from_raw_parts(rid_or_ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }

    pub fn stat(&self) -> Result<CounterGroupStat, String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;

        perf_counter_group_stat(ret_area_ptr, self.rid);

        let [is_ok, ptr, len] = ret_area;
        match is_ok {
            1 => {
                let stat: CounterGroupStat = unsafe { claim_raw_parts_de(ptr as _, len as _) };
                Ok(stat)
            }
            0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }
}

impl Drop for CounterGroup {
    fn drop(&mut self) {
        if self.is_dropped.not() {
            profiling_prelude_intrinsics::drop_resource(self.rid);
        }
    }
}
