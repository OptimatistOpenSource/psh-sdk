use crate::bindings::op::*;
use crate::counting::CounterStat;
use alloc::boxed::Box;
use alloc::string::String;
use core::mem::MaybeUninit;
use profiling_prelude_perf_types::claim_raw_parts_de;

pub struct CounterGuard {
    pub(crate) rid: u32,
}

impl CounterGuard {
    pub fn event_id(&self) -> Result<u64, String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;

        perf_counter_guard_event_id(ret_area_ptr, self.rid);

        let [is_ok, ptr, len] = ret_area;
        match is_ok {
            1 => {
                let event_id = unsafe { Box::<u64>::from_raw(ptr as _) };
                Ok(*event_id)
            }
            0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }

    pub fn stat(&self) -> Result<CounterStat, String> {
        #[allow(invalid_value)]
        let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
        let ret_area_ptr = ret_area.as_ptr() as _;

        perf_counter_guard_stat(ret_area_ptr, self.rid);

        let [is_ok, ptr, len] = ret_area;
        match is_ok {
            1 => {
                let stat: CounterStat = unsafe { claim_raw_parts_de(ptr as _, len as _) };
                Ok(stat)
            }
            0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
            _ => unreachable!(),
        }
    }
}

impl Drop for CounterGuard {
    fn drop(&mut self) {
        profiling_prelude_intrinsics::drop_resource(self.rid);
    }
}
