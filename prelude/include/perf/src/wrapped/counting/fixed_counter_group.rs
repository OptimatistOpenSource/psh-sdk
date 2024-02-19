use crate::bindings::op::*;
use alloc::string::String;
use core::mem::MaybeUninit;
use profiling_prelude_perf_types::claim_raw_parts_de;
use profiling_prelude_perf_types::counting::CounterGroupStat;

pub struct FixedCounterGroup {
    pub(crate) rid: u32,
}

impl FixedCounterGroup {
    pub fn enable(&self) -> Result<(), String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;

        perf_fixed_counter_group_enable(ret_area_ptr, self.rid);

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

        perf_fixed_counter_group_disable(ret_area_ptr, self.rid);

        let [is_ok, ptr, len] = ret_area;
        match is_ok {
            1 => Ok(()),
            0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }

    pub fn reset(&self) -> Result<(), String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;

        perf_fixed_counter_group_reset(ret_area_ptr, self.rid);

        let [is_ok, ptr, len] = ret_area;
        match is_ok {
            1 => Ok(()),
            0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }

    pub fn stat(&self) -> Result<CounterGroupStat, String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;

        perf_fixed_counter_group_stat(ret_area_ptr, self.rid);

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

impl Drop for FixedCounterGroup {
    fn drop(&mut self) {
        profiling_prelude_intrinsics::drop_resource(self.rid)
    }
}
